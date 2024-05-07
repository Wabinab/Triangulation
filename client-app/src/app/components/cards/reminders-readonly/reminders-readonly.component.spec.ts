import { ComponentFixture, TestBed } from '@angular/core/testing';

import { RemindersReadonlyComponent } from './reminders-readonly.component';

describe('RemindersReadonlyComponent', () => {
  let component: RemindersReadonlyComponent;
  let fixture: ComponentFixture<RemindersReadonlyComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [RemindersReadonlyComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(RemindersReadonlyComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
