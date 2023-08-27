use crate::{
    dns_types::{Class, RecordType},
    message::Message,
};
use bitvec::macros::internal::funty::Fundamental;
use clap::Parser;
use rand::Rng;
use std::time;

mod cli_parser_clap;
mod cli_read_resolve;
mod dns_stats;
mod dns_types;
mod io;
mod message;
mod parse;
mod stop_handler;
mod time_stamp;

fn main() {
    let cmd_args = cli_parser_clap::CmdArgs::parse();
    if cmd_args.verbose > 0 {
        println!("Debug: Done with CmdArgs {:?}", cmd_args);
        cli_read_resolve::print_resolve_conf();
    };
    let cli_parser_clap::CmdArgs {
        //unpack struct 🤯
        name,
        record_type,
        resolver,
        interval,
        verbose,
        mut slow,
    } = cmd_args;
    let mut firsttime = true;
    let mut stats = dns_stats::Stats::new(slow);
    let stop = stop_handler::Stop::new();

    while ((interval > 0) && !stop.stopped()) | firsttime {
        firsttime = false;
        let query_id = rand::thread_rng().gen();
        let msg = Message::new_query(query_id, &name, record_type).unwrap();
        let timer = time::Instant::now();
        let mut output: String = String::new();
        match io::send_req(msg, resolver, verbose) {
            Err(e) => {
                let total_fails = stats.fail(1);
                println!("Error {total_fails} send_req: {e}");
            }
            Ok((resp, number_of_bytes, _src_addr)) => {
                let duration = timer.elapsed().as_millis().as_u64();
                stats.update(duration);
                output += stats.gen_output().as_str();
                match io::gen_resp(resp, number_of_bytes, query_id, resolver, verbose) {
                    Err(e) => {
                        let total_fails = stats.fail(1);
                        println!("Error {total_fails} print_resp: {e}");
                        output += format!("Error io::print_resp: {e}").as_str();
                    }
                    Ok(out_s) => {
                        output += out_s.as_str();
                    }
                }
                println!("{}", output);
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
    if (verbose > 0) || (interval > 0) {
        println!("The End.");
    }
}
