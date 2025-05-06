use std::{fs, io::Read};

use craftflow_nbt::DynNBT;
use rust_mc_proto::{prelude::*, write_packet, MCConnTcp, Packet, ProtocolError};
use uuid::Uuid;


pub trait ReadWriteNBT<T>: DataReader + DataWriter {
    fn read_nbt(&mut self) -> Result<T, ProtocolError>;
    fn write_nbt(&mut self, val: &T) -> Result<(), ProtocolError>;
}

impl ReadWriteNBT<DynNBT> for Packet {
    fn read_nbt(&mut self) -> Result<DynNBT, ProtocolError> {
        let mut data = Vec::new();
        let pos = self.get_ref().position();
        self.get_mut()
            .read_to_end(&mut data)
            .map_err(|_| ProtocolError::StringParseError)?;
        let (remaining, value) =
            craftflow_nbt::from_slice(&data).map_err(|_| ProtocolError::StringParseError)?;
        self.get_mut()
            .set_position(pos + (data.len() - remaining.len()) as u64);
        Ok(value)
    }

    fn write_nbt(&mut self, val: &DynNBT) -> Result<(), ProtocolError> {
        craftflow_nbt::to_writer(self.get_mut(), val).map_err(|_| ProtocolError::StringParseError)?;
        Ok(())
    }
}


fn main() -> Result<(), ProtocolError> {
    let mut conn = MCConnTcp::connect("localhost:25565").unwrap();

    conn.write_packet(&Packet::build(0x00, |packet| {
        packet.write_varint(770)?;
        packet.write_string("localhost")?;
        packet.write_unsigned_short(25565)?;
        packet.write_varint(2)
    })?)?;

    conn.write_packet(&Packet::build(0x00, |packet| {
        packet.write_string("TheMixRay")?;
        packet.write_uuid(&Uuid::default())
    })?)?;

    loop {
        let mut packet = conn.read_packet()?;
            
        if packet.id() == 0x03 {
            let threshold = packet.read_varint()?;

            if threshold >= 0 {
                conn.set_compression(Some(threshold as usize));
            }
        } else if packet.id() == 0x02 {
            break;
        }
    }

    conn.write_packet(&Packet::empty(0x03))?;

    conn.write_packet(&Packet::build(0x02, |packet| {
        packet.write_string("minecraft:brand")?;
        packet.write_string("vanilla")
    })?)?;

    conn.write_packet(&Packet::build(0x00, |packet| {
        packet.write_string("en_us")?;
        packet.write_signed_byte(12)?;
        packet.write_varint(0)?;
        packet.write_boolean(true)?;
        packet.write_byte(127)?;
        packet.write_varint(1)?;
        packet.write_boolean(true)?;
        packet.write_boolean(true)?;
        packet.write_varint(0)
    })?)?;

    let mut packet = conn.read_packet()?; // server brand

    let id = packet.read_string()?;
    println!("message id: {}", id);
    println!("message data: {}", String::from_utf8_lossy(&packet.get_bytes()[id.len()+1..]));

    let mut packet = conn.read_packet()?; // feature flags

    let flags_len = packet.read_varint()?;

    println!("got {} feature flags:", flags_len);

    for _ in 0..flags_len {
        let flag = packet.read_string()?;

        println!("flag: {}", flag);
    }

    let mut packet = conn.read_packet()?; // wait for known packs packet

    if packet.id() != 0x0E {
        println!("got unexpected packet while looking for 0x0E: 0x{:02X}", packet.id());
        return Ok(());
    }

    let packs_len = packet.read_varint()?;

    println!("got {} known packs:", packs_len);

    for _ in 0..packs_len {
        println!("{}:{} v{}", packet.read_string()?, packet.read_string()?, packet.read_string()?);
    }

    packet.set_id(0x07); // make it serverbound

	conn.write_packet(&packet)?;

    let mut data = Vec::new();

    loop {
        let mut packet = conn.read_packet()?;

        if packet.id() != 0x07 { // update tags
            let registries_len = packet.read_varint()?;

            println!("got update tags: {}", registries_len);

            for _ in 0..registries_len {
                let registry = packet.read_string()?;

                println!("registry: {}", registry);

                let tags_len = packet.read_varint()?;

                for _ in 0..tags_len {
                    let tag_name = packet.read_string()?;

                    println!("tag: {}", tag_name);

                    let entries_len = packet.read_varint()?;

                    for _ in 0..entries_len {
                        let entry = packet.read_varint()?;

                        println!("entry: {}", entry);
                    }
                }
            }

            fs::write("update-tags.bin", packet.get_bytes()).unwrap();

            break;
        }

        println!("got registry: {}", packet.read_string()?);

        let entries_len = packet.read_varint()?;

        for _ in 0..entries_len {
            let entry_id = packet.read_string()?;
            let has_data = packet.read_boolean()?;

            if has_data {
                let entry_data = packet.read_nbt()?;

                println!("entry: {}, data: {:?}", entry_id, entry_data);
            } else {
                println!("entry: {}, no data", entry_id);
            }
        }

        write_packet(&mut data, None, 0, &packet)?;
    }

    fs::write("registry-data.bin", &data).unwrap();

    let packet = conn.read_packet()?;
    conn.write_packet(&packet)?; // finish conf

    loop {
        let mut packet = conn.read_packet()?;

        if packet.id() == 0x41 {
            let id = packet.read_varint()?;

            conn.write_packet(&Packet::build(0x00, |packet| packet.write_varint(id))?)?;
        }

        if packet.id() == 0x27 {
            // here you can read "Chunk Data and Update Light" packet

            break;
        }
    }

    Ok(())
}
