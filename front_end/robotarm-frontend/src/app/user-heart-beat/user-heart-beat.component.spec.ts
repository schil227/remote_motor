/* tslint:disable:no-unused-variable */
import { async, ComponentFixture, TestBed } from '@angular/core/testing';
import { By } from '@angular/platform-browser';
import { DebugElement } from '@angular/core';

import { UserHeartBeatComponent } from './user-heart-beat.component';

describe('UserHeartBeatComponent', () => {
  let component: UserHeartBeatComponent;
  let fixture: ComponentFixture<UserHeartBeatComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ UserHeartBeatComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(UserHeartBeatComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
