use rppal::gpio::Gpio;

use rppal::gpio::Trigger;
use std::sync::{Arc, RwLock, Mutex};
use std::thread;
use std::time::Duration;
use std::net::TcpStream;
use std::io::Write;

use crate::scoreboard_runner::ScoreboardRunner;

pub fn run_scoreboard(goal_count: Arc<RwLock<u8>>) {
    let mut runner = ScoreboardRunner::create_scoreboard_runner();

    let runner_count = Arc::clone(&goal_count);
    thread::spawn(move || runner.run_scoreboard(runner_count));
    let handler = thread::spawn(|| listen_for_basket(goal_count));

    handler.join().unwrap();
}

fn listen_for_basket(score: Arc<RwLock<u8>>) {
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

        let score_value = increment_score(&score);

        let handle = thread::spawn(move || {
            update_webserver(score_value);
        });

        thread::sleep(Duration::from_millis(300));

        if basket_switch.is_low() {
            println!("Switch may be stuck!");

            println!("eat one input for switch to change.");
            basket_switch.poll_interrupt(true, None).unwrap();
            thread::sleep(Duration::from_millis(300));
        }

        handle.join().expect("Failed to join `update_server` thread!");
    }
}

fn increment_score(score: &Arc<RwLock<u8>>) -> u8{
    let mut score = score.write().unwrap();
    *score = *score + 1;

    if *score > 99 {
        *score = 0;
    }

    *score
}

fn update_webserver(score: u8) {
    let mut stream = TcpStream::connect("192.168.1.248:7871").expect("Failed to establish TCP connection to webapi (192.168.1.248:7871)");

    let mut buf = [0; 1];
    bincode::serialize_into(&mut buf[..], &score).unwrap();

    stream.write(&buf).unwrap();
}
