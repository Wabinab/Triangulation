import { Component, Input, inject } from '@angular/core';
import { SharedModule } from '../../../shared/shared.module';
import { SharedFormsModule } from '../../../shared/shared-forms.module';
import { FontAwesomeModule } from '@fortawesome/angular-fontawesome';
import { NgbActiveModal, NgbModal } from '@ng-bootstrap/ng-bootstrap';
import { faCheck } from '@fortawesome/free-solid-svg-icons';
import { FormArray, FormBuilder, FormGroup } from '@angular/forms';
import { Http3Service } from '../../../services/http3.service';
import { TranslateService } from '@ngx-translate/core';
import { ToastrService } from 'ngx-toastr';
import { Routes } from '../../../models/routes';

@Component({
  selector: 'app-checklist-readonly',
  standalone: true,
  imports: [SharedModule, SharedFormsModule, FontAwesomeModule],
  templateUrl: './checklist-readonly.component.html',
  styleUrl: './checklist-readonly.component.scss'
})
export class ChecklistReadonlyComponent {
  bsModalRef = inject(NgbActiveModal);
  // private modalSvc = inject(NgbModal);

  faTick = faCheck;

  @Input() id: number = -1;
  @Input() curr_stage: number = 0;
  @Input() filename: string = "";

  items: any;
  loading = true;
  public myForm: FormGroup;

  constructor(private http3: Http3Service, private fb: FormBuilder,
    private translate: TranslateService, private toastr: ToastrService
  ) {
    this.myForm = fb.group({
      title: [''],
      checklist: fb.array([])
    });
    this.myForm.disable();
    setTimeout(() => this.loadData(), 75);
  }

  async loadData() {
    let data = {
      filename: this.filename,
      stage_index: this.curr_stage,
      pipeline_index: this.id
    };
    this.loading = true;
    this.http3.send(Routes.SamplePi, JSON.stringify(data)).then((res: any) => {
      this.items = this.http3.json_handler(res);
      this.myForm.get('title')?.setValue(this.items.title);
      this.set_checklist(this.items.others);
      this.loading = false;
    }).catch(err => { this.doErr(err); this.loading = false; });
  }

  private set_checklist(checklist: any) {
    let c = this.myForm.get('checklist') as FormArray;
    if ([null, undefined].includes(checklist)) { this.loading = false; return; }
    checklist.forEach((d: string) => {
      c.push(this.fb.group({
        title: [d ?? '']
      }));
    });
    // checklist.disable();
  }

  // ==============================================
  get checklists() {
    return this.myForm.get('checklist')!.value.map((c: any) => c.title);
  }

  // get checklists() {
  //   const q = this.myForm.get('checklist') as FormArray;
  //   return q['controls'];
  // }

  // ==============================================
  doErr(err: any) {
    console.error(err);
    if (typeof(err) === 'string') this.toastr.error(this.translate.instant(err || ''));
    else this.toastr.error(err);
  }

  cancel() {
    this.bsModalRef.dismiss();
  }
}
