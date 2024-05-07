import { Component, Input, inject } from '@angular/core';
import { SharedModule } from '../../../shared/shared.module';
import { SharedFormsModule } from '../../../shared/shared-forms.module';
import { FontAwesomeModule } from '@fortawesome/angular-fontawesome';
import { NgbActiveModal, NgbModal } from '@ng-bootstrap/ng-bootstrap';
import { FormBuilder, FormGroup } from '@angular/forms';
import { Http3Service } from '../../../services/http3.service';
import { TranslateService } from '@ngx-translate/core';
import { ToastrService } from 'ngx-toastr';
import { Routes } from '../../../models/routes';

@Component({
  selector: 'app-kelly-readonly',
  standalone: true,
  imports: [SharedModule, SharedFormsModule, FontAwesomeModule],
  templateUrl: './kelly-readonly.component.html',
  styleUrl: './kelly-readonly.component.scss'
})
export class KellyReadonlyComponent {
  bsModalRef = inject(NgbActiveModal);
  // private modalSvc = inject(NgbModal);
  // param_latest = { value: 100 };

  @Input() id: number = -1;
  @Input() curr_stage: number = 0;
  @Input() filename: string = '';

  items: any;
  loading = true;
  public myForm: FormGroup;

  constructor(private http3: Http3Service, private fb: FormBuilder,
    private translate: TranslateService, private toastr: ToastrService
  ) {
    this.myForm = fb.group({
      title: ['']
    });
    this.myForm.disable();
    setTimeout(() => this.loadData(), 50);
  }

  async loadData() {
    let data = {
      filename: this.filename,
      stage_index: this.curr_stage,
      pipeline_index: this.id
    };
    this.loading = true;
    this.http3.send(Routes.SamplePi, JSON.stringify(data)).then(async (res: any) => {
      this.items = this.http3.json_handler(res);
      this.myForm.get('title')?.setValue(this.items.title);
      this.loading = false;
    }).catch(err => { this.doErr(err); this.loading = false; });
  }

  // ============================================
  doErr(err: any) {
    console.error(err);
    if (typeof(err) === 'string') this.toastr.error(this.translate.instant(err || ''));
    else this.toastr.error(err);
  }

  cancel() {
    this.bsModalRef.dismiss();
  }
}
