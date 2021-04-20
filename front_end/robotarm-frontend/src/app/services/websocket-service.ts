import { Injectable, OnInit} from '@angular/core'
import { webSocket, WebSocketSubject} from 'rxjs/webSocket'

// const ip : string = "96.42.97.100";
const ip : string = "192.168.1.248";

export class Foo{
    public foo: string = "";
}

@Injectable({
    providedIn: 'root'
  })
export class WebsocketService implements OnInit{
    private subject: WebSocketSubject<Foo> = webSocket(
            {
                url: "ws://" + ip + ":8001",
            });

    constructor(){}

    ngOnInit(): void {
    }

    getSubject(): WebSocketSubject<Foo> {
        return this.subject;
    }
}