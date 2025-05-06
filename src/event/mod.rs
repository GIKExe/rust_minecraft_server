use rust_mc_proto::Packet;

use super::{ServerError, player::context::ClientContext, protocol::ConnectionState};
use std::sync::Arc;

#[macro_export]
macro_rules! generate_handlers {
    ($name:ident $(, $arg_ty:ty)* $(,)?) => {
        paste::paste! {
            fn [<on_ $name _priority>](&self) -> i8 {
                0
            }

            fn [<on_ $name>](&self, _: Arc<ClientContext> $(, _: $arg_ty)*) -> Result<(), ServerError> {
                Ok(())
            }
        }
    };
}

/// Пример использования:
///
///     trigger_event!(client, status, &mut response, state);
#[macro_export]
macro_rules! trigger_event {
    ($client:ident, $event:ident $(, $arg_ty:expr)* $(,)?) => {{
        paste::paste! {
            for handler in $client.server.listeners(
                |o| o.[<on_ $event _priority>]()
            ).iter() {
                handler.[<on_ $event>](
                    $client.clone()
                    $(, $arg_ty)*
                )?;
            }
        }
    }};
}

pub trait Listener: Sync + Send {
	generate_handlers!(status, &mut String);
	generate_handlers!(plugin_message, &str, &[u8]);
}

pub trait PacketHandler: Sync + Send {
	generate_handlers!(incoming_packet, &mut Packet, &mut bool, ConnectionState);
	generate_handlers!(outcoming_packet, &mut Packet, &mut bool, ConnectionState);
	generate_handlers!(state, ConnectionState);
}
