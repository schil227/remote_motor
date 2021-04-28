import { Injectable, OnInit} from '@angular/core'
import { webSocket, WebSocketSubject} from 'rxjs/webSocket'

import { Command } from '../models/command';
import { ServerState, WebsocketMessage } from '../models/websocket'

import { environment } from '../../environments/environment';

@Injectable({
    providedIn: 'root'
  })
export class WebsocketService implements OnInit{
    private subject: WebSocketSubject<WebsocketMessage> = webSocket(
        {
        url: "ws://" + environment.serverIp + ":8001",
        deserializer: msg =>  { 
            let data = JSON.parse(msg.data);
            
            // Convert the string to an enum
            data.state = ServerState[data.state];

            return data;
        }});

    constructor(){}

    ngOnInit(): void {
    }

    sendPing(): void {
        const message = new WebsocketMessage();

        this.subject.next(message);
    }

    getSubject(): WebSocketSubject<WebsocketMessage> {
        return this.subject;
    }
}