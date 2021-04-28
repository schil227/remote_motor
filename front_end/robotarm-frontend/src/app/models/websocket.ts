import {Command} from './command'

export enum ServerState {
    AcceptingInput,
    Warning,
    Locked
}

export class WebsocketMessage{
    public state: ServerState = ServerState.AcceptingInput;
    public command: Command = new Command();
}
