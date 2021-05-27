use crate::models::command_models::CommandData;

#[derive(Debug, Copy, Clone)]
pub struct CommandDataSummation {
    aggregate_score: usize,
    total_data_points: usize
}

impl CommandDataSummation {
    fn new() -> CommandDataSummation {
        CommandDataSummation{
            aggregate_score: 0,
            total_data_points: 0
        }
    }

    fn new_set() -> [CommandDataSummation; 5] {
        [CommandDataSummation::new(); 5]
    }

    fn add_data_point(&mut self, command_value: usize){
        if command_value != 0 {
            self.aggregate_score += command_value;
            self.total_data_points += 1;
        }
    }

    fn return_aggregate(&self) -> u8{
        if self.total_data_points == 0 {
            return 0;
        }

        return (self.aggregate_score / self.total_data_points) as u8;
    }
}

pub fn aggregate_commands(commands : Vec<CommandData>) -> CommandData {
    let mut aggregate = CommandDataSummation::new_set();

    for command in commands.into_iter() {
        aggregate[0].add_data_point(command.claw as usize);
        aggregate[1].add_data_point(command.hand as usize);
        aggregate[2].add_data_point(command.forearm as usize);
        aggregate[3].add_data_point(command.strongarm as usize);
        aggregate[4].add_data_point(command.shoulder as usize);
    };

    let averaged : Vec<u8> = aggregate
        .iter()
        .map(|x| x.return_aggregate())
        .collect();

    CommandData{
        claw : averaged[0],
        hand : averaged[1],
        forearm : averaged[2],
        strongarm : averaged[3],
        shoulder : averaged[4]
    }
}