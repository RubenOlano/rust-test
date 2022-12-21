// Local thread to use for the times
// Create an object to store the times

use std::cell::RefCell;

pub struct Times {
    times: RefCell<Vec<f64>>,
}

impl Times {
    pub fn new() -> Self {
        Times {
            times: RefCell::new(Vec::new()),
        }
    }

    pub fn add_time(&self, time: f64) {
        self.times.borrow_mut().push(time);
    }

    pub fn avg_time(&self) -> f64 {
        let times = self.times.borrow();
        let sum: f64 = times.iter().sum();
        sum / times.len() as f64
    }

    pub fn remove_larger_times(&self) {
        let avg = self.avg_time();
        self.times.borrow_mut().retain(|e| (*e - avg).abs() < 500.0);
    }

    pub fn len(&self) -> usize {
        self.times.borrow().len()
    }
}
