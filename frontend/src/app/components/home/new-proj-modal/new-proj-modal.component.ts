import { Component, inject, Output, EventEmitter } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { SharedModule } from '../../../shared/shared.module';
import { SharedFormsModule } from '../../../shared/shared-forms.module';
import { NgbModal } from '@ng-bootstrap/ng-bootstrap';
import { CancellationComponent } from '../../cancellation/cancellation.component';
import { TranslateService } from '@ngx-translate/core';

@Component({
  selector: 'app-new-proj-modal',
  standalone: true,
  imports: [SharedModule, SharedFormsModule],
  templateUrl: './new-proj-modal.component.html',
  styleUrl: './new-proj-modal.component.scss'
})
export class NewProjModalComponent {
  myForm: FormGroup;
  templates: any[] = [];
  loading: boolean = false;
  submitting: boolean = false;
  desc_limit = 300;

  @Output() emitCallback = new EventEmitter<any>();
  private modalSvc = inject(NgbModal);

  constructor(private fb: FormBuilder, private translate: TranslateService) {
    this.myForm = this.fb.group({
      name: ['', [Validators.required, Validators.maxLength(50)]],
      description: ['', [Validators.maxLength(this.desc_limit)]],
      template_uuid: ['', [Validators.required]]  // more validation later. 
    });
  }

  
  onSubmit() {
    if (!this.myForm.valid || this.loading || this.submitting) return;
    this.submitting = true;
    const row = {
      name: this.myForm.get('name')!.value,
      description: this.myForm.get('description')!.value,
      template_uuid: this.myForm.get('template_uuid')!.value,
    };
  }

  modalCancel: any;
  cancel() {
    if (this.loading || this.submitting) return;
    if (this.is_dirty()) {
      // Use cancellation modal which'll emit to parent component. 
      this.modalCancel = this.modalSvc.open(CancellationComponent);
      this.modalCancel.componentInstance.back_path = "hide modal";
      this.modalCancel.closed.subscribe((res: any) => {
        // console.log(res);
        res["isClosed"] = true;  // from close not dismiss. 
        this.emitCallback.emit(res);
      });
      this.modalCancel.dismissed.subscribe((res: any) => {
        res["isClosed"] = false;
        // this.emitCallback.emit(res);
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
    // Waiting for errSvc. 
  }

  charcount: string = '';
  textCounter(event: any) {
    const charcount = this.desc_limit - event.target.value.length;
    const translate_word = charcount >= 0 ? 'newTempl.charRemain' : 'newTempl.charOver';
    this.translate.get(translate_word, {}).subscribe((res: string) => {
      this.charcount = `${Math.abs(charcount)} ${res}`;
    });
  }

  // ==================================================
  // Fetch templates. 

  // Template example: {uuid: 'some_uuid', name: "Meh"}


  
  // ==================================================
  // Debug

  //  get errors() {
  //   const myerrors: any = {};
  //   Object.keys(this.myForm.controls).forEach(key => {
  //     // Get errors of every form control
  //     const form = this.myForm.get(key)!;
  //     if (form.errors != null && (form.dirty || form.touched)) { 
  //       myerrors[key] = form.errors; 
  //     }
  //   });

  //   return Object.keys(myerrors).length ? myerrors : null;
  // }

  // public findInvalidControls() {
  //   const invalid = [];
  //   const controls = this.myForm.controls;
  //   for (const name in controls) {
  //       if (controls[name].invalid) {
  //           invalid.push(controls[name].errors);
  //       }
  //   }
  //   return invalid;
  // }
}
