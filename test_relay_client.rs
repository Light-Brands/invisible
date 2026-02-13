//! Simple test client to verify relay node is receiving packets

use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    println!("📡 Testing Invisible Relay Node...");
    println!("Connecting to 127.0.0.1:8080\n");

    // Create UDP socket
    let socket = UdpSocket::bind("127.0.0.1:0")?;
    let relay_addr = "127.0.0.1:8080";

    // Send test packet
    let test_message = b"PING - Test packet from client";
    socket.send_to(test_message, relay_addr)?;

    println!("✅ Sent test packet: {:?}", String::from_utf8_lossy(test_message));
    println!("   Size: {} bytes", test_message.len());
    println!("\n💡 Check relay node logs - you should see 'Received packet' with size={}", test_message.len());

    Ok(())
}
