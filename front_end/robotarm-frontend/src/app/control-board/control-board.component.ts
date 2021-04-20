import { Component, OnInit } from '@angular/core';
import { Control } from '../models/control'
import { VideoStream } from '../models/video-stream'
import { Command } from '../models/command'
import {catchError, map, tap} from 'rxjs/operators'
import { WebApiService} from '../services/webapi-service'
import { Foo, WebsocketService} from '../services/websocket-service'
import { Observer } from 'rxjs'

@Component({
  selector: 'app-control-board',
  templateUrl: './control-board.component.html',
  styleUrls: ['./control-board.component.css']
})
export class ControlBoardComponent implements OnInit {
    controls : Control[]  = Control.InitalControls();
    streams : VideoStream[] = VideoStream.VideoStreams();

    observer : Observer<Foo> = {
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

        this.socket.getSubject().subscribe(this.observer)
    }

    issueCommand(){
        let command = Command.create(this.controls);

        this.api.pushCommands(command);
    }
}