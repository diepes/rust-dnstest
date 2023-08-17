use crate::{
    cli::AppArgs,
    dns_types::{Class, RecordType},
    message::Message,
};
use bitvec::macros::internal::funty::Fundamental;
use rand::Rng;
use std::time;

mod cli;
mod dns_stats;
mod dns_types;
mod io;
mod message;
mod parse;
mod stop_handler;

const VERBOSE: bool = false;

fn main() {
    let AppArgs {
        name,
        record_type,
        resolver,
        interval,
    } = AppArgs::parse().expect("Failed to parse command line arguments");
    let mut firsttime = true;
    let mut stats = dns_stats::Stats::new();
    let stop = stop_handler::Stop::new();

    while ((interval > 0) && !stop.stopped()) | firsttime {
        firsttime = false;
        let query_id = rand::thread_rng().gen();
        let msg = Message::new_query(query_id, &name, record_type).unwrap();
        let timer = time::Instant::now();
        match io::send_req(msg, resolver, VERBOSE) {
            Err(e) => {
                let total_fails = stats.fail(1);
                println!("Error {total_fails} send_req: {e}");
            }
            Ok((resp, number_of_bytes, _src_addr)) => {
                let duration = timer.elapsed().as_millis().as_i64();
                stats.update(duration);
                stats.print();
                if let Err(e) = io::print_resp(resp, number_of_bytes, query_id, resolver, VERBOSE) {
                    println!("Error io::print_resp: {e}");
                }
                println!();
            }
        }
        //# if sleep interval > 1 check every second for ctrl-c
        if interval < 2 {
            std::thread::sleep(std::time::Duration::from_secs(interval));
        } else {
            for _ in 0..interval {
                if !stop.stopped() {
                    std::thread::sleep(std::time::Duration::from_secs(1));
                } else {
                    break;
                }
            }
        }
    }
    println!("The End.");
}
