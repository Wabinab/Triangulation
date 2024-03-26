import { Component, inject, Input } from '@angular/core';
import { Router } from '@angular/router';
import { NgbActiveModal } from '@ng-bootstrap/ng-bootstrap';
import { TranslateModule } from '@ngx-translate/core';

@Component({
  selector: 'app-cancellation',
  standalone: true,
  imports: [TranslateModule],
  templateUrl: './cancellation.component.html',
  styleUrl: './cancellation.component.scss'
})
export class CancellationComponent {
  bsModalRef = inject(NgbActiveModal);
  // @Output() emitCallback = new EventEmitter<any>();

  @Input() title: string = 'cancellation.Save';
  @Input() back_dismiss: boolean = false;  // dismiss instead of close when calling back(). 
  @Input() back_path: any;

  constructor(private router: Router) {}

  done() {
    // this.bsModalRef.dismiss('Cross click');
    this.back();
  }

  back() {
    if (this.back_path == "hide modal") {
      if (!this.back_dismiss) { this.bsModalRef.close({ ty: 'hide' }); return; }
      this.bsModalRef.dismiss({ ty: 'hide' });
      // this.emitCallback.emit({ ty: 'hide' });
    } else {
      if (!this.back_dismiss) this.bsModalRef.close({});
      else this.bsModalRef.dismiss({});
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
