import { Component, Input, inject } from '@angular/core';
import { SharedModule } from '../../../shared/shared.module';
import { SharedFormsModule } from '../../../shared/shared-forms.module';
import { FontAwesomeModule } from '@fortawesome/angular-fontawesome';
import { NgbActiveModal, NgbModal } from '@ng-bootstrap/ng-bootstrap';
import { faAdd, faCheck, faXmark } from '@fortawesome/free-solid-svg-icons';
import { faSave } from '@fortawesome/free-regular-svg-icons';
import { FormArray, FormBuilder, FormGroup, Validators } from '@angular/forms';
import { Http3Service } from '../../../services/http3.service';
import { TranslateService } from '@ngx-translate/core';
import { ToastrService } from 'ngx-toastr';
import { Routes } from '../../../models/routes';
import { CancellationComponent } from '../../cancellation/cancellation.component';

@Component({
  selector: 'app-checklist',
  standalone: true,
  imports: [SharedModule, SharedFormsModule, FontAwesomeModule],
  templateUrl: './checklist.component.html',
  styleUrl: './checklist.component.scss'
})
export class ChecklistComponent {
  bsModalRef = inject(NgbActiveModal);
  private modalSvc = inject(NgbModal);

  faAdd = faAdd;
  faCross = faXmark;
  faSave = faSave;
  faTick = faCheck;

  @Input() id: number = -1;
  @Input() curr_stage: number = 0;
  @Input() filename: string = '';

  items: any;
  loading = true;
  submitting = false;
  is_new = true;
  public myForm: FormGroup;

  constructor(private http3: Http3Service, private fb: FormBuilder,
    private translate: TranslateService, private toastr: ToastrService
  ) {
    this.myForm = fb.group({
      title: [, [Validators.required, Validators.minLength(1), Validators.maxLength(50)]],
      checklist: fb.array([])
    });

    setTimeout(() => this.loadData(), 100);
  }

  async loadData() {
    let data = {
      filename: this.filename,
      stage_index: this.curr_stage,
      pipeline_index: this.id
    };

    let value: any = await this.http3.send(Routes.Pi, JSON.stringify(data));
    this.items = JSON.parse(value ?? '{}');
    if (this.items.err && this.items.err == "backend.OOBPipeline") {
      this.is_new = true;
      // this.add_to_list();  // temporary. 
      this.loading = false; return;
    }
    this.is_new = false;
    if (this.items.err && this.items.err.length > 0) {
      this.doErr(this.items.err);
      this.loading = false; return;
    }
    this.myForm.get('title')?.setValue(this.items.title);
    this.set_checklist(this.items.others);
    // this.loading = false;  // set inside set_checklist. 
  }

  private set_checklist(checklist: any) {
    let c = this.myForm.get('checklist') as FormArray;

    if ([null, undefined].includes(checklist)) {this.loading = false; return; }
    checklist.forEach((d: string) => {
      c.push(this.fb.group({
        title: [d ?? '', [Validators.required, Validators.minLength(1), Validators.maxLength(1_000)]]
      }));
    })
    this.loading = false;
  }

  // ===============================================
  onSubmit() {
    if (this.submitting || this.loading || this.myForm.invalid) {
      if (this.myForm.invalid) this.doErr("err.InvalidForm"); return;
    }
    this.submitting = true;
    const row = {
      filename: this.filename,
      stage_index: this.curr_stage,
      pipeline_index: this.id,
      title: this.myForm.get('title')!.value,
      checklist: this.get_checklist()
    };
    console.warn(row);

    this.http3.send(this.is_new ? Routes.PiNew2 : Routes.PiEdit2, JSON.stringify(row))
    .then((res: any) => {
      this.submitting = false;
      this.bsModalRef.close({ ty: this.http3.json_handler(res) });
    }).catch((err: any) => { this.doErr(err); this.submitting = false; });
  }

  modalCancel: any;
  cancel() {
    console.log(`Loading: ${this.loading}\nSubmitting: ${this.submitting}`);
    if (this.loading || this.submitting) return;
    if (this.is_dirty()) {
      this.modalCancel = this.modalSvc.open(CancellationComponent);
      this.modalCancel.componentInstance.back_path = "hide modal";
      this.modalCancel.componentInstance.back_dismiss = true;
      this.modalCancel.closed.subscribe((_: any) => {
        this.onSubmit(); 
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
      if (field.dirty) dirty = true;
    });

    // TBD: Check second level later. 
    return dirty;
  }

  // ===============================================
  get_checklist() {
    // Need convert to array.
    return this.myForm.get('checklist')!.value.map((c: any) => c.title);
  }
  
  get checklists() {
    const q = this.myForm.get('checklist') as FormArray;
    return q['controls'];
  }

  add_to_list() {
    let c = this.myForm.get('checklist') as FormArray;
    c.push(this.fb.group({
      title: ['', [Validators.required, Validators.minLength(1), Validators.maxLength(1_000)]]
    }));
    return c.length;
  }

  add_and_focus(i: number | null = null, event: any = null) {
    if (i !== null && (event.value.length === 0 || event.value === null)) return;
    let length = this.add_to_list();
    setTimeout(() => {
      document.getElementById(`title_${i ?? length-1}`)!.focus();
    }, 10);
  }

  remove_item(i: number) {
    let c = this.myForm.get('checklist') as FormArray;
    c.removeAt(i);
  }
  
  // ===============================================
  doErr(err: any) {
    console.error(err);
    if (typeof(err) === 'string') this.toastr.error(this.translate.instant(err || ''));
    else this.toastr.error(err);
    // this.translate.get(err).subscribe((res: any) => this.toastr.error(res), 
    //   err => this.toastr.error(err));
  }
}
