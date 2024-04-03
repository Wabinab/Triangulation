import { ComponentFixture, TestBed } from '@angular/core/testing';

import { KellyProjComponent } from './kelly-proj.component';

describe('KellyProjComponent', () => {
  let component: KellyProjComponent;
  let fixture: ComponentFixture<KellyProjComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [KellyProjComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(KellyProjComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
