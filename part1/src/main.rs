use std::net::IpAddr;
use pnet::{
	transport::{TransportChannelType, TransportProtocol, transport_channel, icmpv6_packet_iter},
	packet::{
		Packet,
		icmpv6::{Icmpv6Types::{EchoRequest, EchoReply}, Icmpv6Packet},
		ip::IpNextHeaderProtocols::Icmpv6
	}
};

const DEFERRED_MS: u64 = 0x20f7;
const CODE: &[u8; 8] = b"76823362";
const ICMP_HEADER_LEN: usize = 4;
const ECHO_HEADER_LEN: usize = 4;

#[tokio::main]
async fn main() {
	let (mut tx, mut rx) = match transport_channel(
		4096,
		TransportChannelType::Layer4(TransportProtocol::Ipv6(Icmpv6))
	) {
		Ok(x) => x,
		Err(e) => panic!("Failed to create the channel: {}", e)
	};

	let (mpsc_tx, mut mpsc) = tokio::sync::mpsc::channel::<(IpAddr, [u8; 4])>(10);
	let task_tx = tokio::spawn(async move {
		let mut buf = [0u8; 16];
		buf[0] = EchoReply.0;
		buf[(ICMP_HEADER_LEN + ECHO_HEADER_LEN)..].copy_from_slice(CODE);
		while let Some((addr, param)) = mpsc.recv().await {
			buf[ICMP_HEADER_LEN..(ICMP_HEADER_LEN + ECHO_HEADER_LEN)].copy_from_slice(&param);
			let new = Icmpv6Packet::new(&buf[..]).unwrap();
			match tx.send_to(new, addr) {
				Ok(n) => println!("Sent an ICMP echo reply with {} bytes payload to {}", n, addr),
				Err(e) => eprintln!("Failed to send reply to {}: {}", addr, e)
			}
		}
	});

	let task_rx = tokio::spawn(async move {
		let mut rx_iter = icmpv6_packet_iter(&mut rx);
		println!("Hongbao is ready.");
		loop {
			if mpsc_tx.is_closed() {
				break;
			}
			match rx_iter.next() {
				Ok((packet, addr)) => {
					if packet.get_icmpv6_type() != EchoRequest { continue; }
					let payload = packet.payload();
					if payload.len() < ECHO_HEADER_LEN { continue; }
					println!("Received an ICMP echo request from {} with ID and Sequence# {:02x?}", addr, &payload[..ECHO_HEADER_LEN]);
					let param: [u8; 4] = payload[..ECHO_HEADER_LEN].try_into().unwrap();

					let mpsc = mpsc_tx.clone();
					tokio::spawn(async move {
						use std::time::Duration;
						use tokio::time::sleep;
						sleep(Duration::from_millis(DEFERRED_MS)).await;
						_ = mpsc.send((addr, param)).await;
					});
				},
				Err(e) => panic!("Failed to receive a packet: {}", e)
			}
		}
	});

	tokio::select! {
		_ = task_tx => { eprintln!("TX task exited"); }
		_ = task_rx => { eprintln!("RX task exited"); }
	}
	eprintln!("Bye!");
}