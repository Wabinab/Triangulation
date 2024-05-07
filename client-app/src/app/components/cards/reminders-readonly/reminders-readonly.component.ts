import { Component, Input, inject } from '@angular/core';
import { SharedModule } from '../../../shared/shared.module';
import { SharedFormsModule } from '../../../shared/shared-forms.module';
import { FontAwesomeModule } from '@fortawesome/angular-fontawesome';
import { HumanPipe } from '../../../directives/human.pipe';
import { NgbActiveModal } from '@ng-bootstrap/ng-bootstrap';
import { AnswerTypes } from '../../../models/answer-types';
import { AbstractControl, FormArray, FormBuilder, FormGroup } from '@angular/forms';
import { Http3Service } from '../../../services/http3.service';
import { TranslateService } from '@ngx-translate/core';
import { ToastrService } from 'ngx-toastr';
import { CardTypes } from '../../../models/card-types';
import { Routes } from '../../../models/routes';
import { KeyValue } from '@angular/common';

@Component({
  selector: 'app-reminders-readonly',
  standalone: true,
  imports: [SharedModule, SharedFormsModule, FontAwesomeModule, HumanPipe],
  templateUrl: './reminders-readonly.component.html',
  styleUrl: './reminders-readonly.component.scss'
})
export class RemindersReadonlyComponent {
  bsModalRef = inject(NgbActiveModal);

  originalOrder = (a: KeyValue<string,AnswerTypes>, b: KeyValue<string,AnswerTypes>): number => {
    return 0;
  }

  @Input() id: number = -1;
  @Input() curr_stage: number = 0;
  @Input() filename: string = '';

  AnswerTypes = AnswerTypes;

  items: any;
  loading: boolean = true;
  public myForm: FormGroup;

  constructor(private http3: Http3Service, private fb: FormBuilder,
    private translate: TranslateService, private toastr: ToastrService
  ) {
    this.myForm = fb.group({
      t: [CardTypes.Reminders],
      title: [''],
      questions: fb.array([])
    });
    this.myForm.disable();

    setTimeout(() => this.get_pipeline_item_by_id(), 100);
  }

  async get_pipeline_item_by_id() {
    let data = {
      filename: this.filename,
      stage_index: this.curr_stage,
      pipeline_index: this.id
    };
    this.loading = true;
    this.http3.send(Routes.SamplePi, JSON.stringify(data)).then(async (res: any) => {
      this.items = this.http3.json_handler(res);
      await this.loadData();  // loading false inside loadData()
    }).catch(err => { this.doErr(err); this.loading = false; });
  }

  async loadData() {
    if (this.myForm.get('t')!.value != this.items.ty) {
      this.doErr("reminder.MismatchCard"); return;
    }
    this.myForm.get('title')?.setValue(this.items.title);
    this.set_row();
    this.loading = false;
  }

  // ==============================================
  set_row() {
    this.items.others.forEach((q: any) => {
      this.add_new_question(q);
    })
  }

  add_new_question(data: any = {}) {
    let qs = this.myForm.get('questions') as FormArray;
    qs.push(this.fb.group({
      question: [data.q ?? ''],
      q_type: [data.t ?? AnswerTypes.Long],
      rows: this.fb.array([]),
      cols: this.fb.array([]),
      min: [data.min ?? 1],
      max: [data.max ?? 5],
      min_name: [data.min_name ?? ''],
      max_name: [data.max_name ?? '']
    }));
    if (data.r) { data.r.forEach((c: string, i: number) => this.add_rowcol(qs.length - 1, i, 'rows', c)); }
    if (data.c) { data.c.forEach((c: string, i: number) => this.add_rowcol(qs.length - 1, i, 'cols', c)); }
    qs.disable();
  }

  get questions() {
    const q = this.myForm.get('questions') as FormArray;
    return q['controls'];
  }

  is_qtype(i: number, value: string | string[]) {
    const q = this.get_q('questions', i);
    if (typeof value == 'object') return value.includes(q.get('q_type')!.value);
    return q.get('q_type')!.value == value;
  }

  add_rowcol(i: number, j: number, rowcol: string, data = '') {
    let mcqs = this.get_formarray('questions', i, rowcol);
    mcqs.push(this.fb.group({
      option: [data]
    }));
  }

  rowcols(i: number, rowcol = 'rows') {
    const mcqs = this.get_formarray('questions', i, rowcol);
    return mcqs['controls'];
  }

  // =========================================
  range(size:number, startAt:number = 0) : ReadonlyArray<number> {
    return [...Array(size).keys()].map(i => i + startAt);
  }

  private get_formarray(first: string, i: number, second: string ): FormArray {
    let q = this.get_q(first, i);
    return q.get(second) as FormArray;
  }

  private get_q(first: string, i: number): AbstractControl {
    let qs = this.myForm.get(first) as FormArray;
    return qs.at(i);
  }

  range_min_max(min: number, max: number) : ReadonlyArray<number> {
    const size = Math.abs(max - min + 1);
    const startAt = min;
    return this.range(size, startAt);
  }

  // ==============================================
  cancel() {
    this.bsModalRef.dismiss();
  }

  doErr(err: any, t_opt: any = {}) {
    console.error(err);
    if (typeof(err) === 'string') this.toastr.error(this.translate.instant(err || '', t_opt));
    else this.toastr.error(err);
  }
}
