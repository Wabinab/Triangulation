import { TestBed } from '@angular/core/testing';

import { TranslateManagerService } from './translate-manager.service';

describe('TranslateManagerService', () => {
  let service: TranslateManagerService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(TranslateManagerService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
