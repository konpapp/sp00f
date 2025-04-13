use std::net::IpAddr;

use pnet::datalink::{self, Channel::Ethernet};
use pnet::packet::{
    Packet, ethernet::EthernetPacket, ip::IpNextHeaderProtocols, ipv4::Ipv4Packet, udp::UdpPacket,
};

const SELECTED_INTERFACE: &str = "eth0";

fn main() {
    println!("looking for network interface {}", SELECTED_INTERFACE);
    let interfaces = datalink::interfaces();

    #[cfg(debug_assertions)]
    {
        let interface_names_and_desc: Vec<(String, String)> = interfaces
            .iter()
            .map(|interface| (interface.name.clone(), interface.description.clone()))
            .collect();

        println!(
            "list of network interfaces:\n{:?}\n",
            interface_names_and_desc
        );
    }

    let interface = interfaces
        .into_iter()
        .find(|interface| interface.name == SELECTED_INTERFACE)
        .expect("selected interface not found");

    let local_ip = interface
        .ips
        .iter()
        .find_map(|ip| match ip.ip() {
            IpAddr::V4(ipv4) => Some(ipv4),
            _ => None,
        })
        .expect("no IPv4 address found for the selected interface");

    println!(
        "{} interface found with local IP {:?}!",
        SELECTED_INTERFACE, local_ip
    );

    let (_tx, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("unhandled channel type"),
        Err(e) => panic!("failed to create datalink channel: {}", e),
    };

    println!(
        "[*] listening for DNS (UDP port 53) packets from {} on {}",
        local_ip, SELECTED_INTERFACE
    );

    loop {
        match rx.next() {
            Ok(frame) => {
                let packet = EthernetPacket::new(frame).unwrap();

                if let Some(ip_packet) = Ipv4Packet::new(packet.payload()) {
                    if ip_packet.get_next_level_protocol() == IpNextHeaderProtocols::Udp
                        && ip_packet.get_source() == local_ip
                    {
                        let udp_packet = UdpPacket::new(ip_packet.payload()).unwrap();
                        if udp_packet.get_destination() == 53 {
                            println!(
                                "[+] captured DNS query: destination={}",
                                ip_packet.get_destination()
                            );
                        }
                    }
                }
            }
            Err(e) => panic!("error reading packet: {}", e),
        }
    }
}
