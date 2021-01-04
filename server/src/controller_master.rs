use super::motor_controller::MotorControlData;
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::Arc;

use models::MotorCommand;
use models::MotorName;

extern crate linux_embedded_hal as hal;
extern crate pwm_pca9685 as pca9685;

use pca9685::{Pca9685, Address};

pub struct ControllerMaster{
    motor_controllers: HashMap<i8, MotorControlData>,
    pub pwm_handle: Arc<Mutex<Pca9685<hal::I2cdev>>>
}

impl ControllerMaster{
    pub fn new() -> ControllerMaster{
        let i2c_device = hal::I2cdev::new("/dev/i2c-1").unwrap();

        // 0x40, the standard address for pwm9685 
        let address = Address::default();
    
        let mut pwm = Pca9685::new(i2c_device, address).unwrap();

        // Initialize all motor channels.
        pwm.set_all_on_off(&[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0], &[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]).unwrap();

        pwm.set_prescale(121).unwrap();

        pwm.enable().unwrap();

        ControllerMaster{
            motor_controllers: HashMap::new(),
            pwm_handle: Arc::new(Mutex::new(pwm))
        }
    }

    pub fn get_controller(&self, motor_name: MotorName) -> Option<&MotorControlData>{
        self.motor_controllers.get(&(motor_name as i8))
    }

    pub fn add_controller(&mut self, motor_name: MotorName, controller: MotorControlData){
        self.motor_controllers.insert(motor_name as i8, controller);
    }

    pub fn command_all_controllers(&self, command: MotorCommand){
        for (_motor_name, controller) in self.motor_controllers.iter(){
            controller.update(command);
        }
    }
}