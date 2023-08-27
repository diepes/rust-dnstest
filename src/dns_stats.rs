use crate::time_stamp::get_timestamp_now;

const HIGH_DNS_RESPONSE: i64 = 200;
pub struct Stats {
    stat_cnt: i64,
    stat_high_cnt: i64,
    stat_high_last_timestamp: String,
    stat_fail: i64,
    stat_min: i64,
    stat_ave_last_100: f64,
    last_duration: i64,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            stat_cnt: 0,
            stat_high_cnt: 0,
            stat_high_last_timestamp: String::from(""),
            stat_fail: 0,
            stat_min: 1000000,
            stat_ave_last_100: 0.0,
            last_duration: 0,
        }
    }

    pub fn update(&mut self, duration: i64) {
        if self.stat_cnt == 0 {
            self.stat_min = duration;
            self.stat_ave_last_100 = duration as f64;
        } else {
            self.stat_ave_last_100 = (self.stat_ave_last_100 * 9.0 + duration as f64) / 10.0;
        }
        self.last_duration = duration;
        self.stat_cnt += 1;
        if duration > HIGH_DNS_RESPONSE {
            self.stat_high_cnt += 1;
            self.stat_high_last_timestamp = get_timestamp_now("");
        } else if self.stat_high_last_timestamp.len() > 0 {
            self.stat_high_last_timestamp = String::from("");
        }
    }

    pub fn fail(&mut self, add: i8) -> i64 {
        self.stat_fail += add as i64;
        self.stat_fail
    }

    pub fn gen_output(&self) -> String {
        let mut output = String::new();
        output += format!("msec:{: <4}", self.last_duration).as_str();
        output += format!("min:{: <4}", self.stat_min).as_str();
        output += format!("ave:{: <6.1}", self.stat_ave_last_100).as_str();
        output += format!("tHigh:{: <4}", self.stat_high_cnt).as_str();
        output += format!("tFail:{: <3}", self.stat_fail).as_str();
        output += format!("Total:{:0>4} ", self.stat_cnt).as_str();
        output += format!("{}", self.stat_high_last_timestamp).as_str();
        // return output
        output
    }
}
