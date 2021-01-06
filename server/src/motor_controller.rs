use models::MotorCommand;
use models::MotorData;
use models::MotorName;

use std::thread;
use std::time::Duration;
use std::sync::Mutex;
use std::sync::Arc;

const STEP: u16 = 2;

extern crate linux_embedded_hal as hal;
extern crate pwm_pca9685 as pca9685;

use pca9685::{Channel, Pca9685};

pub struct MotorControlData{
    runner: Arc<Mutex<MotorRunner>>,
    pwm_handle: Arc<Mutex<Pca9685<hal::I2cdev>>>
}

pub struct MotorRunner{
    data: MotorData,
    motor_channel: Channel,
    target_duty: Mutex<u16>,
    halt: Mutex<bool>
}

impl MotorControlData{
    pub fn start(&self){
        let runner = Arc::clone(&self.runner);
        let handle = Arc::clone(&self.pwm_handle);
        thread::spawn(move || run_motor(runner, handle));
    }
    
    pub fn register(motor_data: MotorData, pwm: &Arc<Mutex<Pca9685<hal::I2cdev>>>) -> Result<MotorControlData, String>{
        let channel = match register_motor(motor_data.motor_name){
            Some(out_pin) => {out_pin},
            None => {
                return Err(format!("Failed obtaining channel for motor: {}", stringify!(motor_data.motor_name)));
            }
        };

        let controller = MotorControlData{
            runner: Arc::new(Mutex::new(MotorRunner{
                motor_channel: channel,
                data: motor_data,
                halt: Mutex::new(true),
                target_duty: Mutex::new(motor_data.min)                
            })),
            pwm_handle: Arc::clone(pwm)
        };

        controller.start();

        Ok(controller)
    }

    pub fn update(&self, command: MotorCommand){
        let runner = &mut Arc::clone(&self.runner);
        update_motor(runner, command);
    }
}

fn register_motor(name: MotorName) -> Option<Channel>{
    match name {
        MotorName::Shoulder => {
            Some(Channel::C0)
        },
        MotorName::StrongArm => {
            Some(Channel::C1)
        },
        MotorName::ForeArm => {
            Some(Channel::C2)
        },
        MotorName::Hand => {
            Some(Channel::C3)
        },
        MotorName::Claw => {
            Some(Channel::C4)
        },
        _ =>{
            None
        }
    }
}

fn run_motor(runner: Arc<Mutex<MotorRunner>>, pwm_handle: Arc<Mutex<Pca9685<hal::I2cdev>>>){
    let mut current_duty: u16;
    let channel: Channel;

    {
        let raw_runner = runner.lock().unwrap();
        channel = (*raw_runner).motor_channel.clone();
        current_duty = raw_runner.data.max
    }

    loop{
        if is_halted(&runner){
            thread::sleep(Duration::from_millis(5));
            continue;
        }

        let target = get_target_duty(&runner);

        println!("current_duty: {} target_duty: {}", current_duty, target);

        match current_duty {
            c if c > target => {
                current_duty = current_duty - STEP;
            },
            c if c < target => {
                current_duty = current_duty + STEP;
            },
            _ => {
                println!("Reached target, halting.");

                {
                    let runner = runner.lock().unwrap();
                    runner.set_halt(true);
                }
    
                continue;
            }
        }

        {
            // motor_pin.lock().unwrap().set_pwm_frequency(HERTZ, current_duty).unwrap();
            let mut handle = pwm_handle.lock().unwrap();
            handle.set_channel_off(channel, current_duty).unwrap();
        }

        thread::sleep(Duration::from_millis(30));
    }
}

fn update_motor(runner: &mut Arc<Mutex<MotorRunner>>, command: MotorCommand){
    match command{
        MotorCommand::Forward(_num) => {
            println!("Moving forward!");

            let target = {
                let raw_runner = runner.lock().unwrap();
                raw_runner.data.max
             };

            set_halt(runner, false);
            update_target_duty(runner, target);
        },
        MotorCommand::Backward(_num) => {
            println!("Moving backward!");

            let target = {
                let raw_runner = runner.lock().unwrap();
                raw_runner.data.min
             };

            set_halt(runner, false);
            update_target_duty(runner, target);
        },
        MotorCommand::Stop() => {
            println!("Halt!");
            set_halt(runner, true);
        }
    }
}

fn is_halted(motor_runner: &Arc<Mutex<MotorRunner>>) -> bool
{
    let control_data = motor_runner.lock().unwrap();
    control_data.is_halted()
}

fn set_halt(motor_runner: &mut Arc<Mutex<MotorRunner>>, halt: bool)
{
    let control_data = motor_runner.lock().unwrap();
    control_data.set_halt(halt);
}

fn update_target_duty(motor_runner: &mut Arc<Mutex<MotorRunner>>, target: u16)
{
    let control_data = &mut motor_runner.lock().unwrap();
    control_data.update_target_duty(target);
}

fn get_target_duty(motor_runner: &Arc<Mutex<MotorRunner>>) -> u16
{
    let control_data = motor_runner.lock().unwrap();
    control_data.get_target_duty()
}

impl MotorRunner{
    pub fn update_target_duty(&mut self, target: u16){
        let mut duty = self.target_duty.lock().unwrap();

        println!("Updating duty target to {}: before {}", target, *duty);
        *duty = target;
        println!("Updating duty target: after {}", *duty);
    }
    
    pub fn get_target_duty(&self) -> u16{
        let duty = self.target_duty.lock().unwrap();
        *duty
    }
    
    pub fn set_halt(&self, is_halted: bool){
        let mut halt_state = self.halt.lock().unwrap();
        *halt_state = is_halted;
    }
    
    pub fn is_halted(&self) -> bool{
        let halt_state = self.halt.lock().unwrap();
        *halt_state
    }
}