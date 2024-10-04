#![allow(dead_code)]

use std::time::Duration;
use std::thread::sleep;
use std::io::{self, Write};


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
        while self.duration > 0 {
            self.is_running = true;

            print!("\r{}", self.duration);
            io::stdout().flush().unwrap();
            self.duration -= 1;
            sleep(Duration::new(1, 0));
        }
        println!("Work done!");
    }

    fn start(&mut self) {
        self.is_running = true;
        self.display_timer();
    } 
}

fn prompt_for_duration() -> u32 {
    let mut user_input = String::new();
    
    println!("Enter how many time you want to work in seconds :");

    io::stdin().read_line(&mut user_input).expect("Failed to read user input");
    let timer = user_input.trim().parse().expect("Failed to parse user input");

    timer
}

fn main() {
    let timer = prompt_for_duration();

    let mut pomodoro = Timer::new(timer);

    Timer::start(&mut pomodoro);
}
