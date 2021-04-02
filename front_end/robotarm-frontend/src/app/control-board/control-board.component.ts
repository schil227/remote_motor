import { Component, OnInit } from '@angular/core';
import { Control } from '../models/control'
import { VideoStream } from '../models/video-stream'
import { Command } from '../models/command'
import {catchError, map, tap} from 'rxjs/operators'
import { WebApiService} from '../services/webapi-service'

@Component({
  selector: 'app-control-board',
  templateUrl: './control-board.component.html',
  styleUrls: ['./control-board.component.css']
})
export class ControlBoardComponent implements OnInit {
    controls : Control[]  = Control.InitalControls();
    streams : VideoStream[] = VideoStream.VideoStreams();

    constructor(
        private api: WebApiService
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

    }

    issueCommand(){
        let command = Command.create(this.controls);

        this.api.pushCommands(command);
    }
}