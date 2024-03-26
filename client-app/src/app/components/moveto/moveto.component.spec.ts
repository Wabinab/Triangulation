import { ComponentFixture, TestBed } from '@angular/core/testing';

import { MovetoComponent } from './moveto.component';

describe('MovetoComponent', () => {
  let component: MovetoComponent;
  let fixture: ComponentFixture<MovetoComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [MovetoComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(MovetoComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
