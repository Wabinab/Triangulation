import { ComponentFixture, TestBed } from '@angular/core/testing';

import { RemindersProjComponent } from './reminders-proj.component';

describe('RemindersProjComponent', () => {
  let component: RemindersProjComponent;
  let fixture: ComponentFixture<RemindersProjComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [RemindersProjComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(RemindersProjComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
