import { ComponentFixture, TestBed } from '@angular/core/testing';

import { NewTemplModalComponent } from './new-templ-modal.component';

describe('NewTemplModalComponent', () => {
  let component: NewTemplModalComponent;
  let fixture: ComponentFixture<NewTemplModalComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [NewTemplModalComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(NewTemplModalComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
