import { Injectable, OnInit} from '@angular/core'
import { webSocket, WebSocketSubject} from 'rxjs/webSocket'

import { Command } from '../models/command';

const ip : string = "96.42.97.100";
// const ip : string = "192.168.1.248";

export class WebsocketMessage{
    public state: string = "";
    public command: Command = new Command()
}

@Injectable({
    providedIn: 'root'
  })
export class WebsocketService implements OnInit{
    private subject: WebSocketSubject<WebsocketMessage> = webSocket(
        {
        url: "ws://" + ip + ":8001",
        // deserializer: msg =>  msg.data
        });

    constructor(){}

    ngOnInit(): void {
    }

    getSubject(): WebSocketSubject<WebsocketMessage> {
        return this.subject;
    }
}