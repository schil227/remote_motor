use super::motor_controller::MotorControlData;
use std::collections::HashMap;
use models::MotorCommand;

pub struct ControllerMaster{
    motor_controllers: HashMap<u8, MotorControlData>
}

impl ControllerMaster{
    pub fn new() -> ControllerMaster{
        ControllerMaster{
            motor_controllers: HashMap::new()
        }
    }

    pub fn get_controller(&self, pin: u8) -> Option<&MotorControlData>{
        self.motor_controllers.get(&pin)
    }

    pub fn add_controller(&mut self, pin: u8, controller: MotorControlData){
        self.motor_controllers.insert(pin, controller);
    }

    pub fn command_all_controllers(&self, command: MotorCommand){
        for (_pin, controller) in self.motor_controllers.iter(){
            controller.update(command);
        }
    }
}