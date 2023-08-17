//! Doing network IO and printing to the terminal.
use crate::message::{header::ResponseCode, Message, MAX_UDP_BYTES};
use anyhow::{anyhow, Result as AResult};
use std::{
    net::{SocketAddr, UdpSocket},
    time::Duration,
};

/// Sends the given DNS message to the given resolver.
/// Returns the binary response.
pub fn send_req(msg: Message, resolver: SocketAddr, verbose: bool) -> AResult<(Vec<u8>, usize)> {
    // Connect to the DNS resolver
    let local_addr = "0.0.0.0:0";
    let socket =
        UdpSocket::bind(local_addr).expect("io: couldn't bind to a udpsocket on local address");
    socket.set_read_timeout(Some(Duration::from_secs(5)))?;
    if verbose {
        println!("io: Bound to local {}", socket.local_addr()?);
    }
    socket.connect(resolver).expect("io: connect_Error couldn't connect to the DNS resolver");
    if verbose {
        println!("io: Connected to remote {resolver}");
    }

    // Send the DNS resolver the message
    let body = msg.serialize_bytes()?;
    if verbose {
        println!("io: Request size: {} bytes", body.len());
    }
    let bytes_sent = socket.send(&body).expect("io: couldn't send data");
    if bytes_sent != body.len() {
        panic!("io: Only {bytes_sent} bytes, message was probably truncated");
    }

    // Get the resolver's response.
    // Note, you have to actually allocate space to write into.
    // I was originally using an empty vector, but reading into an empty vector always
    // instantly succeeds (by writing nothing), so I was discarding the response.
    // See <https://users.rust-lang.org/t/empty-response-from-udp-recv-w-tokio-and-futures/20241/2>
    let mut response_buf = vec![0; MAX_UDP_BYTES];
    match socket.recv(&mut response_buf) {
        Ok(received) => Ok((response_buf, received)),
        Err(e) => Err(anyhow!("io: recv function failed: {:?}", e)),
    }
}

/// Parse the binary response into a DNS message, and print it nicely.
pub fn print_resp(
    resp: Vec<u8>,
    len: usize,
    sent_query_id: u16,
    resolver: SocketAddr,
    verbose: bool,
) -> AResult<()> {
    if verbose {
        println!("io: Response size: {len} bytes");
        println!("{resp:?}");
    }

    // Parse and validate the response.
    let input = resp[..len].to_vec();
    let response_msg = match Message::deserialize(input) {
        Ok(msg) => msg,
        Err(e) => anyhow::bail!("io: Error parsing response: {e}"),
    };
    let received_query_id = response_msg.header.id;
    if sent_query_id != received_query_id {
        eprintln!("io: Mismatch between query IDs. Client sent {sent_query_id} and received {received_query_id}")
    }
    match response_msg.header.resp_code {
        ResponseCode::NoError => {}
        err => anyhow::bail!("io: Error from resolver: {err}"),
    };

    // Reprint the question, why not?
    //print!("Q: ");
    for question in response_msg.question.iter() {
        print!("Q:\"{question}\"");
    }
    print!(" R:\"{}\"", resolver);

    // Print records sent by the resolver.
    match response_msg.answer.len() {
        1 => {
            print!(" Ans:\"{:.<30}\"", response_msg.answer[0].as_dns_response());
        },
        2.. => {
            println!("\nAnswer records:");
            for record in response_msg.answer {
                println!("    {:.<30}", record.as_dns_response());
            }
        }
        _ => (),
    }
    if !response_msg.authority.is_empty() {
        println!("\nAuthority records:");
        for record in response_msg.authority {
            println!("    {}", record.as_dns_response());
        }
    }
    if !response_msg.additional.is_empty() {
        println!("\nAdditional records:");
        for record in response_msg.additional {
            println!("{}", record.as_dns_response());
        }
    }
    Ok(())
}
