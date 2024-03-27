import { ComponentFixture, TestBed } from '@angular/core/testing';

import { KellyComponent } from './kelly.component';

describe('KellyComponent', () => {
  let component: KellyComponent;
  let fixture: ComponentFixture<KellyComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [KellyComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(KellyComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
