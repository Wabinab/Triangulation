import { ComponentFixture, TestBed } from '@angular/core/testing';

import { NewProjModalComponent } from './new-proj-modal.component';

describe('NewProjModalComponent', () => {
  let component: NewProjModalComponent;
  let fixture: ComponentFixture<NewProjModalComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [NewProjModalComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(NewProjModalComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
