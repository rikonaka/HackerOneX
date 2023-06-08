use pnet::datalink::{Channel, MacAddr, NetworkInterface};
use pnet::packet::arp::{ArpHardwareTypes, ArpOperations, ArpPacket, MutableArpPacket};
use pnet::packet::ethernet::EtherTypes;
use pnet::packet::ethernet::MutableEthernetPacket;
use pnet::packet::{MutablePacket, Packet};
use std::net::{IpAddr, Ipv4Addr};
use subnetwork;
// use once_cell::sync::OnceCell;
// static ALIVE_HOSTS: OnceCell<Vec<Vec<String>>> = OnceCell::new();

use crate::Message;

fn get_mac_through_arp(interface: NetworkInterface, target_ip: Ipv4Addr) -> Option<MacAddr> {
    let source_ip = interface
        .ips
        .iter()
        .find(|ip| ip.is_ipv4())
        .map(|ip| match ip.ip() {
            IpAddr::V4(ip) => ip,
            _ => unreachable!(),
        })
        .unwrap();

    let (mut sender, mut receiver) = match pnet::datalink::channel(&interface, Default::default()) {
        Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unknown channel type"),
        Err(e) => panic!("Error happened {}", e),
    };

    let mut ethernet_buffer = [0u8; 42];
    let mut ethernet_packet = MutableEthernetPacket::new(&mut ethernet_buffer).unwrap();

    ethernet_packet.set_destination(MacAddr::broadcast());
    ethernet_packet.set_source(interface.mac.unwrap());
    ethernet_packet.set_ethertype(EtherTypes::Arp);

    let mut arp_buffer = [0u8; 28];
    let mut arp_packet = MutableArpPacket::new(&mut arp_buffer).unwrap();

    arp_packet.set_hardware_type(ArpHardwareTypes::Ethernet);
    arp_packet.set_protocol_type(EtherTypes::Ipv4);
    arp_packet.set_hw_addr_len(6);
    arp_packet.set_proto_addr_len(4);
    arp_packet.set_operation(ArpOperations::Request);
    arp_packet.set_sender_hw_addr(interface.mac.unwrap());
    arp_packet.set_sender_proto_addr(source_ip);
    arp_packet.set_target_hw_addr(MacAddr::zero());
    arp_packet.set_target_proto_addr(target_ip);

    ethernet_packet.set_payload(arp_packet.packet_mut());

    sender
        .send_to(ethernet_packet.packet(), None)
        .unwrap()
        .unwrap();

    for _ in 0..10 {
        let buf = receiver.next().unwrap();
        let arp = ArpPacket::new(&buf[MutableEthernetPacket::minimum_packet_size()..]).unwrap();
        if arp.get_sender_proto_addr() == target_ip
            && arp.get_target_hw_addr() == interface.mac.unwrap()
        {
            // println!("Received reply");
            return Some(arp.get_sender_hw_addr());
        }
    }
    None
}

async fn scan(target_ip: Ipv4Addr, iface_name: &str) {
    let interfaces = pnet::datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .find(|iface| iface.name == iface_name)
        .unwrap();
    let _source_mac = interface.mac.unwrap();

    tokio::spawn(async move {
        let target_mac = get_mac_through_arp(interface, target_ip);
        match target_mac {
            Some(target_mac) => {
                println!("{} MAC address: {}", target_ip, target_mac);
            }
            _ => (),
        }
    });
}


#[tokio::main]
pub async fn run(subnet: &str, interface: &str) {
    // subnet: 192.168.1.0/24
    if subnet.contains("/") {
        let subnet_vec: Vec<&str> = subnet.split("/").collect();
        if subnet_vec.len() == 2 {
            let address = subnet_vec[0];
            let prefix: usize = subnet_vec[1].parse().unwrap();
            let ipv4_iter = subnetwork::ipv4_iter(address, prefix).unwrap();
            for ip in ipv4_iter {
                scan(ip, interface).await;
            }
            return;
        }
    }
    let err = "subnet should like 192.16.1.0/24".to_string();
    err.error_message();
}
