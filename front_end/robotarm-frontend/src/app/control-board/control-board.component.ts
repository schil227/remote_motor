import { Component, OnInit } from '@angular/core';
import { Control } from '../models/control'
import { VideoStream } from '../models/video-stream'
import { Command } from '../models/command'
import {catchError, map, tap} from 'rxjs/operators'
import { WebApiService} from '../services/webapi-service'
import { WebsocketMessage, ServerState} from '../models/websocket'
import { WebsocketService} from '../services/websocket-service'
import { Observer } from 'rxjs'

@Component({
  selector: 'app-control-board',
  templateUrl: './control-board.component.html',
  styleUrls: ['./control-board.component.css']
})
export class ControlBoardComponent implements OnInit {
    controls : Control[]  = Control.InitalControls();
    streams : VideoStream[] = VideoStream.VideoStreams();
    buttonDisabled : boolean = false;
    state: ServerState = ServerState.Locked;

    observer : Observer<WebsocketMessage> = {
        next(v){ 
            console.log('Message received from socket: ' + v)
        },
        error(e){ 
            console.error('Error received from socket: ' + e)
        },
        complete(){ 
            console.log('Socket is closed.') 
        }
    };

    constructor(
        private api: WebApiService,
        private socket: WebsocketService
        ) 
    { 
    }

    ngOnInit() {
        this.api.getCommands().subscribe(
            result => {
                const command : any = result;

                for(let control of this.controls){
                    const value : number = command[control.part.toLowerCase()];
                    control.previousValue = value;
                    control.currentValue = value;
                }
            }
        );

        this.socket.getSubject().subscribe(msg => {
            this.socket.sendPing();

            console.log('Message Received: ' + JSON.stringify(msg));

            const oldState = this.state;
            this.state = msg.state;

            if(oldState == ServerState.Locked){
                const command : any = msg.command;

                for(let control of this.controls){
                    const value : number = command[control.part.toLowerCase()];

                    if(value == 0){
                        continue;
                    }

                    control.previousValue = value;
                    control.currentValue = value;
                }
            }
        });
    }

    aboutToFire() : boolean {
        return this.state == ServerState.Warning;
    }

    locked(): boolean {
        return this.state == ServerState.Locked;
    }

    issueCommand(){
        let command = Command.create(this.controls);

        this.api.pushCommands(command);
    }

    stateHasMessage(){
        return this.state == ServerState.AcceptingInput;
    }

    getButtonText(){
        switch (this.state){
            case ServerState.AcceptingInput:
                return " "; // loadbearing whitespace
            case ServerState.Warning:
                return "Robot arm is about to move"
            case ServerState.Locked:
                return "Robot arm moving";
        }
    }
}