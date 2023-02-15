use std::time::{Duration, Instant};
pub struct Timer {
    simple_timer: Option<SimpleTimer>,
    tmp_remining_time: Option<Duration>,
    pub is_runing: bool,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            simple_timer: None,
            tmp_remining_time: None,
            is_runing: false,
        }
    }

    pub fn start(&mut self, set_time_second: u64) {
        self.tmp_remining_time = Some(Duration::new(set_time_second, 750000000));
        self.simple_timer = Some(SimpleTimer::new(self.tmp_remining_time.unwrap()));
        self.is_runing = true;
    }

    pub fn stop(&mut self) {
        self.simple_timer = None;
        self.tmp_remining_time = None;
        self.is_runing = false;
        println!("call stop")
    }

    pub fn pause(&mut self) {
        self.update_remining_time();
        self.is_runing = false;
    }

    pub fn restart(&mut self) {
        //TODO:これが正しい実装かわからんけどとりあえずmatchする
        // self.simple_timer = Some(SimpleTimer::new(match self.tmp_remining_time {
        //     None => Duration::new(0, 0),
        //     Some(i) => self.tmp_remining_time.unwrap(),
        // }));
        self.simple_timer = match self.tmp_remining_time {
            None => None,
            Some(_i) => Some(SimpleTimer::new(self.tmp_remining_time.unwrap())),
        };
        self.is_runing = true;
    }

    pub fn update_remining_time(&mut self) {
        self.tmp_remining_time = self.remining_time();
    }

    pub fn remining_time(&self) -> Option<Duration> {
        //なぜas_ref()がつくのかいまいち分からん
        //TODO:汚いから治したい一緒にまとめたら異動が発生してだるかった
        match self.is_runing {
            true => match self.simple_timer {
                None => None,
                _ => Some(self.simple_timer.as_ref().expect("REASON").remaining_time()),
            },
            false => match self.tmp_remining_time {
                None => None,
                _ => self.tmp_remining_time,
            },
        }
    }

    pub fn update_time_millis(&self) -> u64 {
        match self.remining_time() {
            None => 200,
            _ => (self.remining_time().unwrap().as_micros() % 200) as u64,
        }
    }
}

struct SimpleTimer {
    end_instant_time: Instant,
}

impl SimpleTimer {
    fn new(alarm_time: Duration) -> SimpleTimer {
        let now_instant_time = Instant::now();

        SimpleTimer {
            end_instant_time: now_instant_time + alarm_time,
        }
    }

    fn remaining_time(&self) -> Duration {
        self.end_instant_time
            .saturating_duration_since(Instant::now())
    }
}
