import { Component, OnInit, Input } from '@angular/core';
import { MatSliderChange } from '@angular/material/slider';
import { Control } from '../models/control'

@Component({
  selector: 'app-control',
  templateUrl: './control.component.html',
  styleUrls: ['./control.component.css']
})
export class ControlComponent implements OnInit {
    @Input() control : Control = Control.default();

    constructor() { }

    ngOnInit() {
    }

    onChange(event: MatSliderChange){
        this.control.hasChanged = true;
        this.control.currentValue = event.value!;
        // console.warn('Part changed', this.control.part, this.control.currentValue);
    }
}
