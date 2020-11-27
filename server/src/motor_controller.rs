use crate::float_comparison::compare;
use crate::float_comparison::FloatComparision;

use models::MotorCommand;
use models::MotorData;
use models::MotorMessage;
use rppal::gpio::OutputPin;
use std::thread;
use std::time::Duration;
use std::sync::Mutex;
use std::sync::Arc;

const HERTZ: f64 = 50.0;
const DUTY_CYCLE_MIN: f64 = 0.02;
const DUTY_CYCLE_MAX: f64 = 0.12;
const DUTY_CYCLE_NEUTRAL: f64 = 0.07;
const STEP: f64 = 0.00055;

pub struct MotorControlData{
    target_duty: Mutex<f64>,
    halt: Mutex<bool>
}

pub fn run_motor(motor_pin: &mut OutputPin, motor_control_data: &mut Arc<Mutex<MotorControlData>>){
    let mut current_duty = DUTY_CYCLE_MIN;
    
    loop{
        if is_halted(motor_control_data){
            motor_pin.clear_pwm().unwrap();
            thread::sleep(Duration::from_millis(5));
            continue;
        }

        let target = get_target_duty(motor_control_data);

        println!("current_duty: {} target_duty: {}", current_duty, target);

        match compare(current_duty, target){
            FloatComparision::GreaterThan =>{
                current_duty = current_duty - STEP;
            },
            FloatComparision::LessThan => {
                current_duty = current_duty + STEP;
            },
            FloatComparision::Equal => {
                println!("Reached target, halting.");
                motor_pin.clear_pwm().unwrap();
    
                {
                    let control_data = motor_control_data.lock().unwrap();
                    control_data.set_halt(true)
                }
    
                continue;
            }
        }

        motor_pin.set_pwm_frequency(HERTZ, current_duty).unwrap();

        thread::sleep(Duration::from_millis(3));
    }
}

pub fn update_motor(message: MotorMessage, motor_control_data: &mut Arc<Mutex<MotorControlData>>){
    match message.command{
        MotorCommand::Forward(num) => {
            println!("Moving forward!");
            set_halt(motor_control_data, false);
            update_target_duty(motor_control_data, DUTY_CYCLE_MAX);
        },
        MotorCommand::Backward(num) => {
            println!("Moving backward!");
            set_halt(motor_control_data, false);
            update_target_duty(motor_control_data, DUTY_CYCLE_MIN);
        },
        MotorCommand::Stop() => {
            println!("Halt!");
            set_halt(motor_control_data, true);
        }
    }
}

fn is_halted(motor_control_data: &mut Arc<Mutex<MotorControlData>>) -> bool
{
    let control_data = motor_control_data.lock().unwrap();
    control_data.is_halted()
}

fn set_halt(motor_control_data: &mut Arc<Mutex<MotorControlData>>, halt: bool)
{
    let control_data = motor_control_data.lock().unwrap();
    control_data.set_halt(halt);
}

fn update_target_duty(motor_control_data: &mut Arc<Mutex<MotorControlData>>, target: f64)
{
    let control_data = motor_control_data.lock().unwrap();
    control_data.update_target_duty(target);
}

fn get_target_duty(motor_control_data: &mut Arc<Mutex<MotorControlData>>) -> f64
{
    let control_data = motor_control_data.lock().unwrap();
    control_data.get_target_duty()
}

impl MotorControlData{
    pub fn new() -> MotorControlData{
        MotorControlData{
            halt: Mutex::new(false),
            target_duty: Mutex::new(DUTY_CYCLE_NEUTRAL)
        }
    }

    pub fn update_target_duty(&self, target: f64){
        let mut duty = self.target_duty.lock().unwrap();
        println!("Updating duty target to {}: before {}", target, *duty);
        *duty = target;
        println!("Updating duty target: after {}", *duty);
    }
    
    pub fn get_target_duty(&self) -> f64{
        let duty =  self.target_duty.lock().unwrap();
        *duty
    }
    
    pub fn set_halt(&self, is_halted: bool){
        let mut halt_state =  self.halt.lock().unwrap();
        *halt_state = is_halted;
    }
    
    pub fn is_halted(&self) -> bool{
        let halt_state =  self.halt.lock().unwrap();
        *halt_state
    }
}