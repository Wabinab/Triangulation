import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ChecklistReadonlyComponent } from './checklist-readonly.component';

describe('ChecklistReadonlyComponent', () => {
  let component: ChecklistReadonlyComponent;
  let fixture: ComponentFixture<ChecklistReadonlyComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ChecklistReadonlyComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(ChecklistReadonlyComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
