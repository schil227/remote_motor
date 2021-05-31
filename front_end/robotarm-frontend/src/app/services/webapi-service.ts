import { Injectable } from '@angular/core';
import { HttpClient, HttpHeaders } from '@angular/common/http'
import {Observable} from 'rxjs';

import { Command } from "../models/command"
import {HeartbeatResponse} from "../models/heartbeat-response"

import { environment } from '../../environments/environment';

@Injectable({
    providedIn: 'root'
  })
export class WebApiService {
    constructor(private http : HttpClient)
    {}

    heartbeat() : Observable<HeartbeatResponse> {
        let headers = new HttpHeaders().append('Content-Type', 'application/json');

        let request = this.http.post<HeartbeatResponse>('http://' + environment.serverIp + ':8000/heartbeat', {}, {
            headers : headers,
            withCredentials : true,
        })

        return request;
    }

    getCommands() : Observable<Command> {
        return this.http.get<Command>('http://' + environment.serverIp + ':8000/command');
    }

    pushCommands(command : Command ) {
        console.warn("request: " + command);

        let headers = new HttpHeaders().append('Content-Type', 'application/json');

        this.http.post<any>('http://' + environment.serverIp + ':8000/set-command', command, {
            headers : headers,
            withCredentials : true,
        })
        .subscribe(
            data => console.log("Command processed successfully." + JSON.stringify(data)),
            error => console.error("Failed to process command. " + error)
        );
    }
}