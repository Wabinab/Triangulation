import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ChecklistProjComponent } from './checklist-proj.component';

describe('ChecklistProjComponent', () => {
  let component: ChecklistProjComponent;
  let fixture: ComponentFixture<ChecklistProjComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ChecklistProjComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(ChecklistProjComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
