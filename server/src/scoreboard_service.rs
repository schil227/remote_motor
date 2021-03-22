use rppal::gpio::Gpio;

use rppal::gpio::Trigger;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::scoreboard_runner::ScoreboardRunner;

pub fn run_scoreboard() {
    let count = Arc::new(Mutex::new(0));

    let mut runner = ScoreboardRunner::create_scoreboard_runner();

    let runner_count = Arc::clone(&count);
    thread::spawn(move || runner.run_scoreboard(runner_count));
    let handler = thread::spawn(|| listen_for_basket(count));

    handler.join().unwrap();
}

fn listen_for_basket(score: Arc<Mutex<u8>>) {
    println!("acquiring pin.");

    let mut basket_switch = Gpio::new()
        .unwrap()
        .get(10)
        .expect("Failed to obtain GPIO pin 10!")
        .into_input_pulldown();

    println!("Set interrupt.");

    basket_switch.set_interrupt(Trigger::FallingEdge).unwrap();

    println!("listening for baskets");

    loop {
        basket_switch.poll_interrupt(true, None).unwrap();

        println!("Scored a basket! yay!");
        increment_score(&score);

        thread::sleep(Duration::from_millis(300));

        if basket_switch.is_low() {
            println!("Switch may be stuck!");

            println!("eat one input for switch to change.");
            basket_switch.poll_interrupt(true, None).unwrap();
            thread::sleep(Duration::from_millis(300));
        }
    }
}

fn increment_score(score: &Arc<Mutex<u8>>) {
    let mut score = score.lock().unwrap();
    *score = *score + 1;

    if *score > 99 {
        *score = 0;
    }
}
