import { Component, OnInit } from '@angular/core';
import { Control } from '../models/control'

@Component({
  selector: 'app-control-board',
  templateUrl: './control-board.component.html',
  styleUrls: ['./control-board.component.css']
})
export class ControlBoardComponent implements OnInit {
    controls : Control[]  = Control.InitalControlls();

    constructor() { }

    ngOnInit() {
    }

    engage(){
        for(const control of this.controls){
            console.warn('Commands:', control.part, control.currentValue);
        }
    }
}
