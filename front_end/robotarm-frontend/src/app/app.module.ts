import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';
import { RouterModule } from '@angular/router';
import { ReactiveFormsModule } from '@angular/forms';
import { HttpClientModule } from '@angular/common/http'
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';
import { MatSliderModule } from '@angular/material/slider';

import { AppComponent } from './app.component';
import { TopBarComponent } from './top-bar/top-bar.component';
import { ControlBoardComponent } from './control-board/control-board.component';
import { ControlComponent } from './control/control.component';
import { DynamicSvgComponent } from './dynamic-svg/dynamic-svg.component';
import { VideoStreamComponent } from './video-stream/video-stream.component';


@NgModule({
  imports: [
    BrowserModule,
    ReactiveFormsModule,
    HttpClientModule,
    RouterModule.forRoot([
      { path: '', component: ControlBoardComponent },
    ]),
    BrowserAnimationsModule,
    MatSliderModule,
  ],
  declarations: [											
    AppComponent,
    TopBarComponent,
    ControlBoardComponent,
    ControlComponent,
    DynamicSvgComponent,
      VideoStreamComponent
   ],
  bootstrap: [
    AppComponent
  ],
//   schemas: [CUSTOM_ELEMENTS_SCHEMA]
})
export class AppModule { }