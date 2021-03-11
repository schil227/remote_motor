use rppal::gpio::Gpio;
use rppal::gpio::OutputPin;

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

const ZERO: [bool; 7] = [true, false, true, true, true, true, true];
const ONE: [bool; 7] = [false, false, false, true, false, false, true];
const TWO: [bool; 7] = [false, true, true, true, true, true, false];
const THREE: [bool; 7] = [false, true, true, true, false, true, true];
const FOUR: [bool; 7] = [true, true, false, true, false, false, true];
const FIVE: [bool; 7] = [true, true, true, false, false, true, true];
const SIX: [bool; 7] = [true, true, true, false, true, true, true];
const SEVEN: [bool; 7] = [false, false, true, true, false, false, true];
const EIGHT: [bool; 7] = [true, true, true, true, true, true, true];
const NINE: [bool; 7] = [true, true, true, true, false, true, true];

pub struct ScoreboardRunner {
    board_pins: [OutputPin; 7],
    c1_pin: OutputPin,
    c2_pin: OutputPin,
}

fn get_digit(number: u8, is_tens_digit: bool) -> u8 {
    if is_tens_digit {
        (number / 10) as u8
    } else {
        number % 10
    }
}

fn get_board_from_number(number: u8) -> [bool; 7] {
    match number {
        0 => ZERO,
        1 => ONE,
        2 => TWO,
        3 => THREE,
        4 => FOUR,
        5 => FIVE,
        6 => SIX,
        7 => SEVEN,
        8 => EIGHT,
        9 => NINE,
        _ => panic!("Invalid number - cannot get board pinout."),
    }
}

impl ScoreboardRunner {
    pub fn create_scoreboard_runner() -> ScoreboardRunner {
        let a_pin = Gpio::new()
            .unwrap()
            .get(4)
            .expect("Failed to obtian GPIO 4 (7-Seg. A)!")
            .into_output();
        let b_pin = Gpio::new()
            .unwrap()
            .get(17)
            .expect("Failed to obtian GPIO 17 (7-Seg. B)!")
            .into_output();
        let c_pin = Gpio::new()
            .unwrap()
            .get(18)
            .expect("Failed to obtian GPIO 18 (7-Seg. C)!")
            .into_output();
        let d_pin = Gpio::new()
            .unwrap()
            .get(22)
            .expect("Failed to obtian GPIO 22 (7-Seg. D)!")
            .into_output();
        let e_pin = Gpio::new()
            .unwrap()
            .get(9)
            .expect("Failed to obtian GPIO 9 (7-Seg. E)!")
            .into_output();
        let f_pin = Gpio::new()
            .unwrap()
            .get(11)
            .expect("Failed to obtian GPIO 11 (7-Seg. F)!")
            .into_output();
        let g_pin = Gpio::new()
            .unwrap()
            .get(7)
            .expect("Failed to obtian GPIO 7 (7-Seg. G)!")
            .into_output();
        let c1_pin = Gpio::new()
            .unwrap()
            .get(8)
            .expect("Failed to obtian GPIO 8 (7-Seg. C1)!")
            .into_output();
        let c2_pin = Gpio::new()
            .unwrap()
            .get(25)
            .expect("Failed to obtian GPIO 25 (7-Seg. C2)!")
            .into_output();

        let board_pins: [OutputPin; 7] = [a_pin, b_pin, c_pin, d_pin, e_pin, f_pin, g_pin];
        ScoreboardRunner {
            board_pins,
            c1_pin,
            c2_pin,
        }
    }

    pub fn run_scoreboard(&mut self, score: Arc<Mutex<u8>>) {
        let mut current_score: u8;

        loop {
            {
                let score = score.lock().unwrap();
                current_score = *score;
            }

            let minor = get_digit(current_score, false);
            let major = get_digit(current_score, true);

            self.update_board(get_board_from_number(minor));

            self.c2_pin.set_low();
            self.c1_pin.set_high();

            thread::sleep(Duration::from_millis(20));

            self.update_board(get_board_from_number(major));

            self.c2_pin.set_high();
            self.c1_pin.set_low();

            thread::sleep(Duration::from_millis(20));
        }
    }

    fn update_board(&mut self, board: [bool; 7]) {
        for (index, pin) in self.board_pins.iter_mut().enumerate() {
            if index > 7 {
                panic!("Index is out of bounds - only 7 pins are supported.")
            }

            if board[index] {
                pin.set_high();
            } else {
                pin.set_low();
            }
        }
    }
}
