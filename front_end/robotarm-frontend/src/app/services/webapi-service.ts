import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http'
import {Observable} from 'rxjs';

import { Command } from "../models/command"

// const ip : string = "96.42.97.100";
const ip : string = "192.168.1.248";

@Injectable({
    providedIn: 'root'
  })
export class WebApiService {
    constructor(private http : HttpClient)
    {}

    getCommands() : Observable<Command> {
        return this.http.get<Command>('http://'+ip+':8000/command');
        // .pipe(result => 
        //     {
        //         return result
        //     }
        // );
    }

    pushCommands(command : Command ) {
        console.warn("request: " + command);

        this.http.post<any>('http://'+ip+':8000/command', command)
        .subscribe(
            data => console.log("Command processed successfully." + data),
            error => console.error("Failed to process command. " + error)
        );
    }
}