use crate::time_stamp::get_timestamp_now;

pub struct Stats {
    stat_cnt: u64,
    stat_high_match: Vec<u64>,
    stat_high_cnt: Vec<u64>,
    stat_high_last_timestamp: String,
    stat_fail: u64,
    stat_min: u64,
    stat_ave_last_100: u64,
    last_duration_usec: u64,
}

impl Stats {
    pub fn new(mut slow: Vec<u64>) -> Self {
        slow.sort_by(|a, b| b.cmp(a)); //High to low
        Self {
            stat_cnt: 0,
            stat_high_cnt: vec![0; slow.len()],
            stat_high_match: slow,
            stat_high_last_timestamp: String::from(""),
            stat_fail: 0,
            stat_min: 1000000,
            stat_ave_last_100: 0,
            last_duration_usec: 0,
        }
    }

    pub fn update(&mut self, duration_usec: u64) {
        if self.stat_cnt == 0 {
            self.stat_min = duration_usec;
            self.stat_ave_last_100 = duration_usec;
        } else {
            self.stat_ave_last_100 = (self.stat_ave_last_100 * 99 + duration_usec) / 100;
        }
        self.last_duration_usec = duration_usec;
        self.stat_cnt += 1;
        if self.stat_min > duration_usec {
            self.stat_min = duration_usec
        }
        if !self.stat_high_last_timestamp.is_empty() {
            // reset timestamp
            self.stat_high_last_timestamp = String::from("");
        }
        for (i, v) in self.stat_high_match.iter().enumerate() {
            if duration_usec > *v * 1000 {
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

    fn to_msec(usec: u64, width: u8) -> String {
        let t = usec as f64;
        let r = t / 1000.0;
        match width {
            0..=2 => {
                if r < 10.0 {
                    let t = r * 10.0;
                    (t.round() / 10.0).to_string()
                } else {
                    r.round().to_string()
                }
            }
            3 => {
                if r < 100.0 {
                    let t = r * 10.0;
                    (t.round() / 10.0).to_string()
                } else {
                    r.round().to_string()
                }
            }
            4.. => {
                if r < 100.0 {
                    let t = r * 100.0;
                    (t.round() / 100.0).to_string()
                } else if r < 1000.0 {
                    let t = r * 10.0;
                    (t.round() / 10.0).to_string()
                } else {
                    r.round().to_string()
                }
            }
        }
    }

    pub fn gen_output(&self) -> String {
        let mut output = String::new();
        output += format!("msec:{: <4}", Self::to_msec(self.last_duration_usec, 2)).as_str();
        output += format!("min:{: <4}", Self::to_msec(self.stat_min, 2)).as_str();
        output += format!("ave:{: <6}", Self::to_msec(self.stat_ave_last_100, 3)).as_str();
        for (i, v) in self.stat_high_match.iter().enumerate() {
            output += format!("t{}:{: <4}", v, self.stat_high_cnt[i]).as_str();
        }
        output += format!("tFail:{: <3}", self.stat_fail).as_str();
        output += format!("Total:{:0>4} ", self.stat_cnt).as_str();
        output += format!("{} ", self.stat_high_last_timestamp).as_str();
        // return output
        output
    }
}
