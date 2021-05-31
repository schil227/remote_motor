import { Control } from '../models/control'
import { VideoStream } from '../models/video-stream'
import { Command } from '../models/command'
import { WebApiService} from '../services/webapi-service'
import { WebsocketMessage, ServerState} from '../models/websocket'
import { WebsocketService} from '../services/websocket-service'

import { Component, OnInit } from '@angular/core';
import { Observer } from 'rxjs'
import { MatSnackBar,  } from '@angular/material/snack-bar'
import { animate, style, state, transition, trigger } from '@angular/animations'

@Component({
  selector: 'app-control-board',
  templateUrl: './control-board.component.html',
  styleUrls: ['./control-board.component.css'],
  animations: [
    trigger('slideDownUp', [
        transition(':enter', [style({height: 0}), animate(250)]),
        transition(':leave', [animate(250, style({height: 0}))])
    ]),
    trigger('rotatedState', [
        state('default', style({ transform: 'rotate(90deg)' })),
        state('rotated', style({ transform: 'rotate(270deg)' })),
        transition('rotated => default', animate('400ms ease-out')),
        transition('default => rotated', animate('400ms ease-in'))
    ])
  ]
})
export class ControlBoardComponent implements OnInit {
    controls : Control[]  = Control.InitalControls();
    streams : VideoStream[] = VideoStream.VideoStreams();
    buttonDisabled : boolean = false;
    state: ServerState = ServerState.Locked;
    goalCount: number = 0;
    showInfo: boolean = true;
    showObjective: boolean = true;
    rotateStateInfo: string = 'default';
    rotateStateObjective: string = 'default';

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
        private socket: WebsocketService,
        private snackBar: MatSnackBar
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
                    control.hasChanged = false;
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
                    control.hasChanged = false;
                }
            }

            if(this.goalCount <  msg.goal_count){
                this.goalCount = msg.goal_count;

                if(msg.goal_count_verified){
                    this.snackBar.open("!! GOAL SCORED !! (Make this cooler)", "", {
                        duration: 4000,
                        panelClass: ['std-snackbar'],
                        verticalPosition: 'bottom',
                        horizontalPosition: 'end'
                    });
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

        this.snackBar.open("Commands Sent!", "", {
            duration: 2000,
            panelClass: ['std-snackbar'],
            verticalPosition: 'bottom',
            horizontalPosition: 'end'
        });

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

    toggleIntro() {
        this.showInfo = !(this.showInfo);
        this.rotateStateInfo = (this.rotateStateInfo === 'default' ? 'rotated' : 'default');
    }

    toggleObjective() {
        this.showObjective = !(this.showObjective);
        this.rotateStateObjective = (this.rotateStateObjective === 'default' ? 'rotated' : 'default');
    }
}