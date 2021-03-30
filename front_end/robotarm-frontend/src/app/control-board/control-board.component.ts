import { Component, OnInit } from '@angular/core';
import { Control } from '../models/control'
import {VideoStream } from '../models/video-stream'
import { HttpClient } from '@angular/common/http'
import {catchError, map, tap} from 'rxjs/operators'

@Component({
  selector: 'app-control-board',
  templateUrl: './control-board.component.html',
  styleUrls: ['./control-board.component.css']
})
export class ControlBoardComponent implements OnInit {
    controls : Control[]  = Control.InitalControls();
    streams : VideoStream[] = VideoStream.VideoStreams();

    constructor(private http: HttpClient) 
    { 
    }

    ngOnInit() {
        this.http.get<any>('http://192.168.1.248:8000/command')
        .subscribe(result => 
            {
                // console.log("Existing Control: claw: " + result.claw + ", hand: " + result.hand + ", forearm: " + result.forearm + ", strongarm: " + result.strongarm + ", shoulder: " + result.shoulder )

                for(let control of this.controls){
                    const value : number = result[control.part.toLowerCase()];
                    control.previousValue = value;
                    control.currentValue = value;
                }

                
            }
        );
    }

    engage(){
        let jsonContent = "{ ";

        for(const control of this.controls){
            if(control.currentValue === control.previousValue){
                continue;
            }
            
            console.warn('Commands:', control.part, control.currentValue);

            jsonContent += control.part + " : " + control.currentValue + ",";
        }

        jsonContent = jsonContent.slice(0, -1) + "}";

        const request = jsonContent;

        console.warn("request: " + request);
    }
}

export class Command {
    public claw : number = 0;
    public hand : number = 0;
    public forearm : number = 0;
    public strongarm : number = 0;
    public shoulder : number = 0;
}