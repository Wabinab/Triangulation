import { ComponentFixture, TestBed } from '@angular/core/testing';

import { KellyReadonlyComponent } from './kelly-readonly.component';

describe('KellyReadonlyComponent', () => {
  let component: KellyReadonlyComponent;
  let fixture: ComponentFixture<KellyReadonlyComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [KellyReadonlyComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(KellyReadonlyComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
