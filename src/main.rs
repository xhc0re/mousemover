// main.rs

use std::env;
use std::thread;
use std::time::{Duration, Instant};

use mouse_rs::Mouse;
use mouse_rs::types::keys::Keys;
use mouse_rs::types::Point;

fn move_mouse() {
    let mouse = Mouse::new();
    // Get the current cursor position (point A)
    let initial_position = mouse.get_position().unwrap();

    // Calculate the destination positions
    let position_b = Point { x: initial_position.x + 10, y: initial_position.y + 10 };
    let position_c = Point { x: initial_position.x + 15, y: initial_position.y - 5 };
    
    // Move the cursor to position B
    thread::sleep(Duration::from_millis(50));
    mouse.move_to(position_b.x, position_b.y).expect("Unable to move mouse");

    // Move the cursor to position C
    thread::sleep(Duration::from_millis(50));
    mouse.move_to(position_c.x, position_c.y).expect("Unable to move mouse");
    

    // Move the cursor back to position A
    thread::sleep(Duration::from_millis(50));
    mouse.move_to(initial_position.x, initial_position.y).expect("Unable to move mouse");

    // Perform a single left mouse button click
    mouse.click(&Keys::LEFT).expect("Unable to click mouse")
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let interval: u64 = match args.get(1) {
        Some(arg) => arg.parse().unwrap_or_else(|_| {
            eprintln!("Invalid time interval provided. Using default interval of 60 seconds.");
            60
        }),
        None => 60,
    };

    loop {
        let start_time = Instant::now();
        move_mouse();
        let elapsed_time = start_time.elapsed();

        if elapsed_time < Duration::from_secs(interval) {
            let sleep_duration = Duration::from_secs(interval) - elapsed_time;
            thread::sleep(sleep_duration);
        }
    }
}
