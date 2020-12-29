use crate::float_comparison::compare;
use crate::float_comparison::FloatComparision;

use models::MotorCommand;
use models::MotorData;
use rppal::gpio::OutputPin;
use rppal::gpio::Gpio;
use std::thread;
use std::time::Duration;
use std::sync::Mutex;
use std::sync::Arc;

const HERTZ: f64 = 50.0;
const STEP: f64 = 0.00055;

pub struct MotorControlData{
    runner: Arc<Mutex<MotorRunner>>
}

pub struct MotorRunner{
    data: MotorData,
    motor_pin: Arc<Mutex<OutputPin>>,
    target_duty: Mutex<f64>,
    halt: Mutex<bool>
}

impl MotorControlData{
    pub fn start(&self){
        let runner = Arc::clone(&self.runner);
        thread::spawn(move || run_motor(runner));
    }
    
    pub fn register(motor_data: MotorData) -> Result<MotorControlData, String>{
        let pin = match register_motor(motor_data.gpio_pin){
            Some(out_pin) => {out_pin},
            None => {
                return Err(format!("Failed obtaining pin {}", motor_data.gpio_pin));
            }
        };

        let controller = MotorControlData{
            runner: Arc::new(Mutex::new(MotorRunner{
                motor_pin: Arc::new(Mutex::new(pin)),
                data: motor_data,
                halt: Mutex::new(true),
                target_duty: Mutex::new(motor_data.min)
            }))
        };

        controller.start();

        Ok(controller)
    }

    pub fn update(&self, command: MotorCommand){
        let runner = &mut Arc::clone(&self.runner);
        update_motor(runner, command);
    }
}

fn register_motor(pin: u8) -> Option<OutputPin>{
    match Gpio::new().unwrap().get(pin) {
        Ok(out_pin) => {
            Some(out_pin.into_output())
        },
        Err(_) =>{
            None
        }
    }
}

fn run_motor(runner: Arc<Mutex<MotorRunner>>){
    let mut current_duty: f64;
    let motor_pin: Arc<Mutex<OutputPin>>;

    {
        let raw_runner = runner.lock().unwrap();
        motor_pin = Arc::clone(&((*raw_runner).motor_pin));
        current_duty = (raw_runner.data.max + raw_runner.data.min) / 2.0;
    }

    loop{
        if is_halted(&runner){
            // {
                // motor_pin.lock().unwrap().clear_pwm().unwrap();
            // }

            thread::sleep(Duration::from_millis(5));
            continue;
        }

        let target = get_target_duty(&runner);

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

                // {
                //     motor_pin.lock().unwrap().clear_pwm().unwrap();
                // }
    
                {
                    let runner = runner.lock().unwrap();
                    runner.set_halt(true)
                }
    
                continue;
            }
        }

        {
            //PWM hardware hookup here
            motor_pin.lock().unwrap().set_pwm_frequency(HERTZ, current_duty).unwrap();
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

fn update_target_duty(motor_runner: &mut Arc<Mutex<MotorRunner>>, target: f64)
{
    let control_data = &mut motor_runner.lock().unwrap();
    control_data.update_target_duty(target);
}

fn get_target_duty(motor_runner: &Arc<Mutex<MotorRunner>>) -> f64
{
    let control_data = motor_runner.lock().unwrap();
    control_data.get_target_duty()
}

impl MotorRunner{
    pub fn update_target_duty(&mut self, target: f64){
        let mut duty = self.target_duty.lock().unwrap();

        println!("Updating duty target to {}: before {}", target, *duty);
        *duty = target;
        println!("Updating duty target: after {}", *duty);
    }
    
    pub fn get_target_duty(&self) -> f64{
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