import { Component, Input, inject } from '@angular/core';
import { SharedModule } from '../../../shared/shared.module';
import { SharedFormsModule } from '../../../shared/shared-forms.module';
import { NgbActiveModal, NgbModal } from '@ng-bootstrap/ng-bootstrap';
import { Http3Service } from '../../../services/http3.service';
import { FormArray, FormBuilder, FormGroup, Validators } from '@angular/forms';
import { TranslateService } from '@ngx-translate/core';
import { ToastrService } from 'ngx-toastr';
// import { UppercaseDirective } from '../../../directives/uppercase.directive';
import { faAdd, faXmark } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeModule } from '@fortawesome/angular-fontawesome';
import { faSave } from '@fortawesome/free-regular-svg-icons';
import { CancellationComponent } from '../../cancellation/cancellation.component';
// import { NumberNodotValidatorDirective } from '../../../directives/number-nodot-validator.directive';
import { Routes } from '../../../models/routes';
// import { NumberNoDotValidator } from '../../directives/number-nodot-validator.directive';

@Component({
  selector: 'app-kelly',
  standalone: true,
  imports: [SharedModule, SharedFormsModule, FontAwesomeModule],
  templateUrl: './kelly.component.html',
  styleUrl: './kelly.component.scss'
})
export class KellyComponent {
  bsModalRef = inject(NgbActiveModal);
  param_latest = { value: 100 };

  faAdd = faAdd;
  faCross = faXmark;
  faSave = faSave;

  @Input() id: number = -1;
  @Input() curr_stage: number = 0;
  @Input() filename: string = '';

  items: any;
  loading = true;
  submitting = false;
  is_new = true;
  public myForm: FormGroup;
  // max_transaction = 5;

  constructor(private http3: Http3Service, private fb: FormBuilder, 
    private translate: TranslateService, private toastr: ToastrService
  ) {
    this.myForm = fb.group({
      // k_perc: [{value: 0, disabled: true}],
      // k_W: [{value: 0, disabled: true}, [Validators.min(0), Validators.max(1)]],
      // k_R: [{value: 1, disabled: true}, [Validators.min(this.min_threshold)]],  // denominator cannot be exactly 0. 
      title: [, [Validators.required, Validators.minLength(1), Validators.maxLength(50)]],
      // transactions: fb.array([])
    });

    // NO AUTOSAVE!!! 
    setTimeout(() => {
      this.loadData();
    }, 100);
  }

  async loadData() {
    let data = {
      filename: this.filename,
      stage_index: this.curr_stage,
      pipeline_index: this.id
    }

    let value: any = await this.http3.send(Routes.Pi, JSON.stringify(data));
    this.items = JSON.parse(value ?? '{}');
    console.warn(this.items);
    if (this.items.err && this.items.err == "backend.OOBPipeline") {
      this.is_new = true;
      this.loading = false;
      return;
    }
    this.is_new = false;
    this.myForm.get('title')?.setValue(this.items.title);
    this.loading = false;
  }

  // ========================================================
  onSubmit() {
    if (this.submitting || this.loading || this.myForm.invalid) {
      if (this.myForm.invalid) {this.translate.get("err.InvalidForm", {})
      .subscribe((res: any) => { this.doErr(res); }); }
      return;
    }
    this.submitting = true;
    const row = {
      filename: this.filename,
      stage_index: this.curr_stage,
      pipeline_index: this.id,
      title: this.myForm.get('title')!.value
    }
    
    this.http3.send(this.is_new ? Routes.PiNew1 : Routes.PiEdit1, JSON.stringify(row))
    .then((res: any) => {
      this.submitting = false;
      this.bsModalRef.close({ ty: this.http3.json_handler(res) });
    }).catch((err: any) => { this.doErr(err); this.submitting = false; });
  }

  private modalSvc = inject(NgbModal);
  modalCancel: any;
  cancel() {
    if (this.loading || this.submitting) return;
    if (this.is_dirty()) {
      this.modalCancel = this.modalSvc.open(CancellationComponent);
      this.modalCancel.componentInstance.back_path = "hide modal";
      this.modalCancel.componentInstance.back_dismiss = true;
      this.modalCancel.closed.subscribe((res: any) => {
        this.onSubmit();  // yes, save (if valid)
        this.bsModalRef.dismiss();
      });
      this.modalCancel.dismissed.subscribe((_: any) => this.bsModalRef.dismiss());
      return;
    }
    this.bsModalRef.dismiss();
  }

  is_dirty() {
    let dirty = false;
    Object.keys(this.myForm.controls).forEach(key => {
      const field = this.myForm.get(key)!;
      if (field.dirty) { dirty = true; }
    });
    return dirty;
  }

  // ===========================================================
  doErr(err: any) {
    console.error(err);
    this.toastr.error(err);
  }
}
