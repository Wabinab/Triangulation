import { Component, EventEmitter, Output, inject } from '@angular/core';
import { SharedModule } from '../../../shared/shared.module';
import { SharedFormsModule } from '../../../shared/shared-forms.module';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { NgbModal } from '@ng-bootstrap/ng-bootstrap';
import { CancellationComponent } from '../../cancellation/cancellation.component';
import { TranslateService } from '@ngx-translate/core';
import { Http3Service } from '../../../services/http3.service';
import { Router } from '@angular/router';
import { ToastrService } from 'ngx-toastr';
import { Routes } from '../../../models/routes';

@Component({
  selector: 'app-new-templ-modal',
  standalone: true,
  imports: [SharedModule, SharedFormsModule],
  templateUrl: './new-templ-modal.component.html',
  styleUrl: './new-templ-modal.component.scss'
})
export class NewTemplModalComponent {
  myForm: FormGroup;
  loading: boolean = false;
  submitting: boolean = false;
  desc_limit = 300;

  @Output() emitCallback = new EventEmitter<any>();
  private modalSvc = inject(NgbModal);

  constructor(private fb: FormBuilder, private translate: TranslateService, 
    private http3: Http3Service, private router: Router, private toastr: ToastrService
  ) {
    this.myForm = this.fb.group({
      name: ['', [Validators.required, Validators.minLength(1), Validators.maxLength(50)]],
      description: ['', [Validators.maxLength(this.desc_limit)]],
      // No edit yet, so no need uuid. Unless we refactor later. 
      // Think first: whether to re-use component or make another. 
    });
    // this.myForm.markAsTouched();
  }

  async onSubmit() {
    if (this.myForm.invalid) { this.doErr("err.InvalidForm"); return; }
    if (this.submitting || this.loading) { this.wait(); return; }
    this.submitting = true; 
    const row = {
      name: this.myForm.get('name')!.value,
      description: this.myForm.get('description')!.value,
      // locale: this.translate.currentLang ?? 'en'
    };
    this.http3.send(Routes.TNew, JSON.stringify(row)).then((data: any) => {
      let filename_json: any = this.http3.json_handler(data);
      this.router.navigate(["/template"], {queryParams: {
        filename: filename_json.filename
      }});
      this.submitting = false;
    }).catch((err) => { this.doErr(err); this.submitting = false; });
    
  }

  modalCancel: any;
  cancel() {
    if (this.loading || this.submitting) return;
    if (this.is_dirty()) {
      this.modalCancel = this.modalSvc.open(CancellationComponent);
      this.modalCancel.componentInstance.back_path = "hide modal";
      this.modalCancel.closed.subscribe((res: any) => {
        res["isClosed"] = true;
        this.emitCallback.emit(res);
      });
      this.modalCancel.dismissed.subscribe((res: any) => {
        res["isClosed"] = false;
      });
      return;
    }
    this.emitCallback.emit({});
  }

  is_dirty() {
    let dirty = false;
    Object.keys(this.myForm.controls).forEach(key => {
      const field = this.myForm.get(key)!;
      if (field.dirty && field.touched) { dirty = true; }
    });

    return dirty;
  }

  doErr(err: any) {
    this.loading = false;
    this.submitting = false;
    console.error(err);
    if (typeof(err) === 'string') this.toastr.error(this.translate.instant(err || ''));
    else this.toastr.error(err);
  }

  wait() {
    this.toastr.info(this.translate.instant("wait"));
  }

  charcount: string = '';
  textCounter(event: any) {
    const charcount = this.desc_limit - event.target.value.length;
    const translate_word = charcount >= 0 ? 'newTempl.charRemain' : 'newTempl.charOver';
    this.charcount = `${Math.abs(charcount)} ${this.translate.instant(translate_word)}`;
  }
}
