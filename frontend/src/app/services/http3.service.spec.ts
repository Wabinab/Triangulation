import { TestBed } from '@angular/core/testing';

import { Http3Service } from './http3.service';

describe('Http3Service', () => {
  let service: Http3Service;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(Http3Service);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
