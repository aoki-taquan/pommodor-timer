use std::time::{Duration, Instant};
use std::rc::Rc;
use std::cell::RefCell;


pub struct  Timer {
    simple_timer: Rc<RefCell<Option<SimpleTimer>>>,
    remining_time: Rc<RefCell<Option<Duration>>>,
    is_runing: Rc<RefCell<bool>>,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            simple_timer: Rc::new(RefCell::new(None)),
            remining_time: Rc::new(RefCell::new(None)),
            is_runing: Rc::new(RefCell::new(false)),
        }
    }

    pub fn start(&  self, set_time_second: u64) {
        *self.remining_time.borrow_mut() = Some(Duration::from_secs(set_time_second));
        *self.simple_timer.borrow_mut() = Some(SimpleTimer::new(self.remining_time.borrow_mut().unwrap()));
        *self.is_runing.borrow_mut() = true;
    }

    pub fn stop(& self) {
        *self.remining_time.borrow_mut() = None;
        *self.simple_timer.borrow_mut() = None;
        *self.is_runing.borrow_mut() = false;
    }

    pub fn pause(& self) {
        self.update_remining_time();
        *self.is_runing.borrow_mut() = false;
    }

    pub fn restart(& self) {
        *self.simple_timer.borrow_mut() = Some(SimpleTimer::new(self.remining_time.borrow_mut().unwrap()));
        *self.is_runing.borrow_mut() = true;
    }

    fn update_remining_time(& self) {
        *self.remining_time.borrow_mut() = Some(self.remining_time());
    }

    pub fn remining_time(&self) -> Duration {
        //なぜas_ref()がつくのかいまいち分からん その後内部可変でこうなった
        self.remining_time.borrow_mut().unwrap()
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