use crate::{
    cli::AppArgs,
    dns_types::{Class, RecordType},
    message::Message,
};
use bitvec::macros::internal::funty::Fundamental;
use rand::Rng;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use ctrlc;
use std::time;

mod cli;
mod dns_types;
mod io;
mod message;
mod parse;

const VERBOSE: bool = false;

fn main() {
    let AppArgs {
        name,
        record_type,
        resolver,
        interval,
    } = AppArgs::parse().expect("Failed to parse command line arguments");
    let mut firsttime = true;
    let mut stat_cnt: i64 = 0;
    let mut stat_max: i64 = 0;
    let mut stat_fail: i64 = 0;
    let mut stat_min: i64 = 100000;
    let mut stat_ave_last_100: f64 = 0.0;

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::Relaxed);
    })
    .expect("Error setting Ctrl-C handler");

    while ((interval > 0 ) && running.load(Ordering::Relaxed)) | firsttime {
        let query_id = rand::thread_rng().gen();
        let msg = Message::new_query(query_id, &name, record_type).unwrap();
        let timer = time::Instant::now();
        // let (resp, len) = io::send_req(msg, resolver, VERBOSE).unwrap();
        let resp: Vec<u8>;
        let len: usize;
        match io::send_req(msg, resolver, VERBOSE) {
            Ok(v) => (resp, len) = v,
            Err(e) => {
                println!("Error: {e}");
                stat_fail += 1;
                std::thread::sleep(std::time::Duration::from_secs(interval));
                continue;
            }
        }
        let duration = timer.elapsed().as_millis().as_i64();
        stat_cnt += 1;
        if firsttime {
            stat_max = duration;
            stat_min = duration;
            stat_ave_last_100 = duration as f64;
            firsttime = false;
        } else {
            if duration > stat_max {
                stat_max = duration
            };
            if duration < stat_min {
                stat_min = duration
            };
            stat_ave_last_100 = (stat_ave_last_100 * 9.0 + duration as f64) / 10.0;
        }
        print!("msec:{: <4}", duration);
        print!(
            "min:{stat_min: <4}max:{stat_max: <4}ave:{stat_ave_last_100: <6.1}cnt:{stat_cnt:0>4} fail:{stat_fail: <3}",
        );
        //io::stdout().flush();
        if let Err(e) = io::print_resp(resp, len, query_id, resolver, VERBOSE) {
            println!("Error: {e}");
        }

        println!();

        std::thread::sleep(std::time::Duration::from_secs(interval));
    }
        println!("The End.");
}
