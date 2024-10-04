#![allow(dead_code)]

use std::time::Duration;
use std::thread::sleep;

#[derive(Debug)]
struct Timer {
    duration: u32,
    elapsed: u32,
    is_running: bool,
}

impl Timer {
    fn new(duration: u32) -> Timer {
        Timer {
            duration,
            elapsed : 0,
            is_running : false,
        }
    }

    fn display_timer(&mut self) {
        //TODO : Add a better display than just println
        while self.duration > 0 {
            self.is_running = true;

            println!("{}", self.duration);
            self.duration -= 1;
            sleep(Duration::new(1, 0));

            if self.duration == 0 {
                self.is_running = false;
            }
        }
    }

    fn start(&mut self) {
        self.is_running = true;
    } 

    fn pause(&mut self) {
        self.is_running = false;
    }

    fn reset(&mut self) {
        self.elapsed = 0;
        self.is_running = false;
    }

    fn update(&mut self) {
        if self.is_running {
            //TODO  
        }
    }
}


fn main() {
    let mut pomodoro = Timer::new(3);

    Timer::display_timer(&mut pomodoro);

    println!("{:#?}", pomodoro);
}
