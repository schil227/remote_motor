import {Command} from './command'

export enum ServerState {
    AcceptingInput,
    Warning,
    Locked
}

export class WebsocketMessage{
    public state: ServerState = ServerState.AcceptingInput;
    public command: Command = new Command();
    public goal_count: number = 0;
    public goal_count_verified : boolean = false;
}
