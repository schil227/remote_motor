import { Component, OnInit } from '@angular/core';
import { VideoStream } from '../models/video-stream'

@Component({
  selector: 'app-testing',
  templateUrl: './testing.component.html',
  styleUrls: ['./testing.component.css']
})
export class TestingComponent implements OnInit {
  streams : VideoStream[] = VideoStream.VideojsStreams();

  constructor() { }

  ngOnInit() {
  }

}
