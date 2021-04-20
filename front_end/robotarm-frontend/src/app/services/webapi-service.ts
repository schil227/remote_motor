import { Injectable } from '@angular/core';
import { HttpClient, HttpHeaders } from '@angular/common/http'
import {Observable} from 'rxjs';

import { Command } from "../models/command"
import {HeartbeatResponse} from "../models/heartbeat-response"

// const ip : string = "96.42.97.100";
const ip : string = "192.168.1.248";

@Injectable({
    providedIn: 'root'
  })
export class WebApiService {
    constructor(private http : HttpClient)
    {}

    heartbeat() : Observable<HeartbeatResponse> {
        let headers = new HttpHeaders().append('Content-Type', 'application/json');

        let request = this.http.post<HeartbeatResponse>('http://'+ip+':8000/heartbeat', {}, {
            headers : headers,
            withCredentials : true,
        })

        return request;
    }

    getCommands() : Observable<Command> {
        return this.http.get<Command>('http://'+ip+':8000/command');
    }

    pushCommands(command : Command ) {
        console.warn("request: " + command);

        this.http.post<any>('http://'+ip+':8000/set-command', command)
        .subscribe(
            data => console.log("Command processed successfully." + data),
            error => console.error("Failed to process command. " + error)
        );
    }
}