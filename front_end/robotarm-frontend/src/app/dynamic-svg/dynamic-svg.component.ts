import { Component, Input, OnInit } from '@angular/core';
import { Control } from '../models/control';
import {DynamicSvg, MovingSvg} from '../models/dynamic-svg';

@Component({
  selector: 'app-dynamic-svg',
  templateUrl: './dynamic-svg.component.html',
  styleUrls: ['./dynamic-svg.component.css']
})
export class DynamicSvgComponent implements OnInit {
    @Input() control : Control = new Control("", 0, 0, new DynamicSvg("", "", "", false, []));

    constructor() { }

    ngOnInit() {
    }

    getRotation(svgData: MovingSvg) : string  {
        const ratio = this.control.currentValue / 100;

        let deg = (svgData.rotationDelta * ratio) - (svgData.rotationDelta / 2)

        if(!svgData.isClockwise){
            deg = -1 * deg
        }

        return deg + "deg"
    }
}
