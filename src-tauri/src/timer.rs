use std::time::{Duration, Instant};



pub struct  Timer {
    simple_timer: Option<SimpleTimer>,
    remining_time: Option<Duration>,
    is_runing: bool,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            simple_timer: None,
            remining_time: None,
            is_runing: false,
        }
    }

    pub fn start(&mut self, set_time_second: u64) {
        self.remining_time = Some(Duration::from_secs(set_time_second));
        self.simple_timer = Some(SimpleTimer::new(self.remining_time.unwrap()));
        self.is_runing = true;
    }

    pub fn stop(&mut self) {
        self.simple_timer = None;
        self.remining_time = None;
        self.is_runing = false;
        println!("call stop")
    }

    pub fn pause(&mut self) {
        self.update_remining_time();
        self.is_runing = false;
    }

    pub fn restart(&mut self) {
        self.simple_timer = Some(SimpleTimer::new(self.remining_time.unwrap()));
        self.is_runing = true;
    }

    fn update_remining_time(&mut self) {
        self.remining_time = Some(self.remining_time());
    }

    pub fn remining_time(&self) -> Duration {
        //なぜas_ref()がつくのかいまいち分からん
        self.simple_timer.as_ref().expect("REASON").remaining_time()
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





// use std::time::{Duration, Instant};
// use std::rc::Rc;
// use std::cell::RefCell;


// pub struct  Timer {
//     simple_timer: Rc<RefCell<Option<SimpleTimer>>>,
//     remining_time: Rc<RefCell<Option<Duration>>>,
//     is_runing: Rc<RefCell<bool>>,
// }

// impl Timer {
//     pub fn new() -> Timer {
//         Timer {
//             simple_timer: Rc::new(RefCell::new(None)),
//             remining_time: Rc::new(RefCell::new(None)),
//             is_runing: Rc::new(RefCell::new(false)),
//         }
//     }

//     pub fn start(&mut  self, set_time_second: u64) {
//         self.remining_time = Rc::new(RefCell::new(Some(Duration::from_secs(set_time_second))));
//         self.simple_timer = Rc::new(RefCell::new(Some(SimpleTimer::new(self.remining_time.borrow_mut().unwrap()))));
//         self.is_runing = Rc::new(RefCell::new(true));
//     }

//     pub fn stop(& self) {
//         self.remining_time = Rc::new(RefCell::new(None));
//         self.simple_timer = Rc::new(RefCell::new(None));
//         self.is_runing = Rc::new(RefCell::new(false));
//     }

//     pub fn pause(& self) {
//         self.update_remining_time();
//         self.is_runing = Rc::new(RefCell::new(false));
//     }

//     pub fn restart(& self) {
//         self.simple_timer = Rc::new(RefCell::new(Some(SimpleTimer::new(self.remining_time.borrow_mut().unwrap()))));
//         self.is_runing = Rc::new(RefCell::new(true));
//     }

//     fn update_remining_time(& self) {
//         self.remining_time = Rc::new(RefCell::new(Some(self.remining_time())));
//     }

//     pub fn remining_time(&self) -> Duration {
//         //なぜas_ref()がつくのかいまいち分からん
//         self.remining_time.borrow_mut().unwrap()
//     }
// }

// struct SimpleTimer {
//     end_instant_time: Instant,
// }

// impl SimpleTimer {
//     fn new(alarm_time: Duration) -> SimpleTimer {
//         let now_instant_time = Instant::now();

//         SimpleTimer {
//             end_instant_time: now_instant_time + alarm_time,
//         }
//     }

//     fn remaining_time(&self) -> Duration {
//         self.end_instant_time
//             .saturating_duration_since(Instant::now())
//     }
// }