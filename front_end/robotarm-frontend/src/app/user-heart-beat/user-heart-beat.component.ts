import { Component, OnInit } from '@angular/core';
import { Observable, interval } from 'rxjs';
import { WebApiService } from '../services/webapi-service';

@Component({
  selector: 'app-user-heart-beat',
  templateUrl: './user-heart-beat.component.html',
  styleUrls: ['./user-heart-beat.component.css']
})
export class UserHeartBeatComponent implements OnInit {
    userCount = 0;

    constructor(private api : WebApiService) { }

    ngOnInit() {
        this.heartbeat();
        interval(5000).subscribe(() => this.heartbeat())
    }

    heartbeat(){
        this.api.heartbeat()
            .subscribe(result => {
                this.userCount = result.user_count;
            })
    }
}
