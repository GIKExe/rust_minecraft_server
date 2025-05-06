#!/usr/bin/env python

# Использование:
# 
#   ./parse_ids.py < Packets.html > src/server/protocol/id.rs

import sys
from bs4 import BeautifulSoup
import re

BOUNDS = ["clientbound", "serverbound"]
MODES = {
    "#Handshaking": "handshake",
    "#Status": "status",
    "#Login": "login",
    "#Play": "play",
    "#Configuration": "configuration"
}

def sanitize_name(name, bound, mode):
    name = (" " + name.lower() + " ").replace(" " + bound.lower() + " ", "").replace(" " + mode.lower() + " ", "") \
        if name.lower() != bound.lower() and name.lower() != mode.lower() else name
    name = re.sub(r'\(.*?\)', '', name)
    name = name.strip()
    name = name.upper()
    name = name.replace(' ', '_')
    return name

def parse_packet_id_table(span):
    table = span.parent.find_next_sibling("table")
    if not table:
        return None
    rows = table.find_all("tr")
    if len(rows) < 2:
        return None
    code_tag = rows[1].find("td").find("code")
    if not code_tag:
        return None
    return code_tag.text.strip()

def main():
    soup = BeautifulSoup(sys.stdin.read(), "html.parser")

    print("/*\n")
    print(" Generated with parse_ids.py \n")
    print(" */\n")

    toc = soup.select_one("#toc")

    for bound_type in BOUNDS:
        print(f"pub mod {bound_type} {{")

        for li in toc.find("ul").find_all("li", recursive=False):
            a = li.find("a", href=True)
            if not a or a["href"] not in MODES:
                continue

            mode = MODES[a["href"]]
            ul = li.find("ul", recursive=False)
            if not ul:
                continue
            lis = ul.find_all("li", recursive=False)
            
            mode_size = 0

            try:
                bound_list = lis[BOUNDS.index(bound_type)].find_all("li")
            except KeyError:
                continue

            for item in bound_list:
                packet_a = item.find("a", href=True)
                if not packet_a or not packet_a["href"].startswith("#"):
                    continue

                href = packet_a["href"].lstrip("#")
                span = soup.find("span", id=href)
                if not span:
                    continue

                packet_id = parse_packet_id_table(span)
                if not packet_id:
                    continue

                name = sanitize_name(" ".join(packet_a.text.split(" ")[1:]), bound_type, mode)
                if len(name) > 0:
                    mode_size += 1

                    if mode_size == 1:
                        print(f"    pub mod {mode} {{")
                    print(f"        pub const {name}: u8 = {packet_id};")
            
            if mode_size > 0:
                print("    }\n")
                
        print("}\n")

if __name__ == "__main__":
    main()