import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';
import { RouterModule } from '@angular/router';
import { ReactiveFormsModule } from '@angular/forms';
import { HttpClientModule } from '@angular/common/http'
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';
import { MatSliderModule } from '@angular/material/slider';
import { MatSnackBarModule } from '@angular/material/snack-bar';

import { AppComponent } from './app.component';
import { TopBarComponent } from './top-bar/top-bar.component';
import { ControlBoardComponent } from './control-board/control-board.component';
import { ControlComponent } from './control/control.component';
import { DynamicSvgComponent } from './dynamic-svg/dynamic-svg.component';
import { VideoStreamComponent } from './video-stream/video-stream.component';
import { VideojsStreamComponent } from './videojs-stream/videojs-stream.component';
import { UserHeartBeatComponent } from './user-heart-beat/user-heart-beat.component';
import { TestingComponent } from './testing/testing.component';

@NgModule({
  imports: [
    BrowserModule,
    ReactiveFormsModule,
    HttpClientModule,
    RouterModule.forRoot([
      { path: '', component: ControlBoardComponent },
      { path: 'test', component: VideojsStreamComponent },
    ]),
    BrowserAnimationsModule,
    MatSliderModule,
    MatSnackBarModule
  ],
  declarations: [
    AppComponent,
    TopBarComponent,
    ControlBoardComponent,
    ControlComponent,
    DynamicSvgComponent,
    VideoStreamComponent,
    VideojsStreamComponent,
    UserHeartBeatComponent,
      TestingComponent
   ],
  bootstrap: [
    AppComponent
  ],
//   schemas: [CUSTOM_ELEMENTS_SCHEMA]
})
export class AppModule { }
