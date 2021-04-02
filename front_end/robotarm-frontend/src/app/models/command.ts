import {Control} from './control'

export class Command {
    public claw : number = 0;
    public hand : number = 0;
    public forearm : number = 0;
    public strongarm : number = 0;
    public shoulder : number = 0;

    public static create(controls : Control[]) : Command{
        let command : any = new Command();

        for(let control of controls){
            if(control.hasChanged){
                command[control.part.toLowerCase()] = control.currentValue;
            }
        }

        return command;
    }
}
