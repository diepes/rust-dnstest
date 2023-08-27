use crate::time_stamp::get_timestamp_now;

const HIGH_DNS_RESPONSE: u64 = 200;
pub struct Stats {
    stat_cnt: u64,
    stat_high_match: Vec<u64>,
    stat_high_cnt: Vec<u64>,
    stat_high_last_timestamp: String,
    stat_fail: u64,
    stat_min: u64,
    stat_ave_last_100: f64,
    last_duration: u64,
}

impl Stats {
    pub fn new(mut slow: Vec<u64>) -> Self {
        slow.sort_by(|a, b| b.cmp(a));  //High to low
        Self {
            stat_cnt: 0,
            stat_high_cnt: vec![0; slow.len()],
            stat_high_match: slow,
            stat_high_last_timestamp: String::from(""),
            stat_fail: 0,
            stat_min: 1000000,
            stat_ave_last_100: 0.0,
            last_duration: 0,
        }
    }

    pub fn update(&mut self, duration: u64) {
        if self.stat_cnt == 0 {
            self.stat_min = duration;
            self.stat_ave_last_100 = duration as f64;
        } else {
            self.stat_ave_last_100 = (self.stat_ave_last_100 * 9.0 + duration as f64) / 10.0;
        }
        self.last_duration = duration;
        self.stat_cnt += 1;
        if self.stat_high_last_timestamp.len() > 0 {
            // reset timestamp
            self.stat_high_last_timestamp = String::from("");
        }
        for (i, v) in self.stat_high_match.iter().enumerate() {
            if duration > *v {
                self.stat_high_cnt[i] += 1;
                self.stat_high_last_timestamp = get_timestamp_now("");
                break;
            }
        }
    }

    pub fn fail(&mut self, add: i8) -> u64 {
        self.stat_fail += add as u64;
        self.stat_fail
    }

    pub fn gen_output(&self) -> String {
        let mut output = String::new();
        output += format!("msec:{: <4}", self.last_duration).as_str();
        output += format!("min:{: <4}", self.stat_min).as_str();
        output += format!("ave:{: <6.1}", self.stat_ave_last_100).as_str();
        for (i, v) in self.stat_high_match.iter().enumerate() {
            output += format!("t{}:{: <4}", v, self.stat_high_cnt[i]).as_str();
        }
        output += format!("tFail:{: <3}", self.stat_fail).as_str();
        output += format!("Total:{:0>4} ", self.stat_cnt).as_str();
        output += format!("{}", self.stat_high_last_timestamp).as_str();
        // return output
        output
    }
}
