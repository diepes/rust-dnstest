pub struct Stats {
    stat_cnt: i64,
    stat_max: i64,
    stat_fail: i64,
    stat_min: i64,
    stat_ave_last_100: f64,
    last_duration: i64,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            stat_cnt: 0,
            stat_max: 0,
            stat_fail: 0,
            stat_min: 1000000,
            stat_ave_last_100: 0.0,
            last_duration: 0,
        }
    }

    pub fn update(&mut self, duration: i64) {
        if self.stat_cnt == 0 {
            self.stat_max = duration;
            self.stat_min = duration;
            self.stat_ave_last_100 = duration as f64;
        } else {
            if duration > self.stat_max {
                self.stat_max = duration
            };
            if duration < self.stat_min {
                self.stat_min = duration
            };
            self.stat_ave_last_100 = (self.stat_ave_last_100 * 9.0 + duration as f64) / 10.0;
        }
        self.last_duration = duration;
        self.stat_cnt += 1;
    }

    pub fn fail(&mut self, add: i8) -> i64 {
        self.stat_fail += add as i64;
        self.stat_fail
    }

    pub fn print(&self) {
        print!("msec:{: <4}", self.last_duration);
        print!("min:{: <4}", self.stat_min);
        print!("max:{: <4}", self.stat_max);
        print!("ave:{: <6.1}", self.stat_ave_last_100);
        print!("cnt:{:0>4} ", self.stat_cnt);
        print!("fail:{: <3}", self.stat_fail);
    }
}
