import { Component, inject, Input } from '@angular/core';
import { Router } from '@angular/router';
import { NgbActiveModal } from '@ng-bootstrap/ng-bootstrap';

@Component({
  selector: 'app-cancellation',
  standalone: true,
  imports: [],
  templateUrl: './cancellation.component.html',
  styleUrl: './cancellation.component.scss'
})
export class CancellationComponent {
  bsModalRef = inject(NgbActiveModal);
  // @Output() emitCallback = new EventEmitter<any>();

  @Input() back_path: any;

  constructor(private router: Router) {}

  done() {
    // this.bsModalRef.dismiss('Cross click');
    this.back();
  }

  back() {
    if (this.back_path == "hide modal") {
      this.bsModalRef.close({ ty: 'hide' });
      // this.emitCallback.emit({ ty: 'hide' });
    } else {
      this.bsModalRef.close({});
      this.router.navigateByUrl(this.back_path);
    }
  }

  // Don't save, cancel instead.
  close_modal() {
    // if (this.back_path == "hide modal") {
    //   this.emitCallback.emit({ ty: 'show this' });
    //   this.bsModalRef.dismiss('Cross click');
    // } else {
    //   this.bsModalRef.dismiss('Cross click');
    // }
    this.bsModalRef.dismiss({ ty: 'show this' });
  }

  save_and_return() {
    this.bsModalRef.close({ ty: 'submit' });
  }
}
