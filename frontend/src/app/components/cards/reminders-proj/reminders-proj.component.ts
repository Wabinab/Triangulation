import { Component, Input, inject } from '@angular/core';
import { SharedModule } from '../../../shared/shared.module';
import { SharedFormsModule } from '../../../shared/shared-forms.module';
import { FontAwesomeModule } from '@fortawesome/angular-fontawesome';
import { HumanPipe } from '../../../directives/human.pipe';
import { NgbActiveModal } from '@ng-bootstrap/ng-bootstrap';
import { AnswerTypes } from '../../../models/answer-types';
import { Http3Service } from '../../../services/http3.service';
import { AbstractControl, FormArray, FormBuilder, FormGroup, Validators } from '@angular/forms';
import { ToastrService } from 'ngx-toastr';

@Component({
  selector: 'app-reminders-proj',
  standalone: true,
  imports: [SharedModule, SharedFormsModule, FontAwesomeModule, HumanPipe],
  templateUrl: './reminders-proj.component.html',
  styleUrl: './reminders-proj.component.scss'
})
export class RemindersProjComponent {
  bsModalRef = inject(NgbActiveModal);

  @Input() id: number = -1;
  @Input() curr_stage: number = 0;
  // @Input() filename: string = '';
  @Input() t_uuid: string = '';
  @Input() t_ver: number = 0;

  AnswerTypes = AnswerTypes;

  items: any;
  loading: boolean = true;
  submitting: boolean = false;
  public myForm: FormGroup;

  constructor(private http3: Http3Service, private fb: FormBuilder,
    private toastr: ToastrService
  ) {
    this.myForm = fb.group({
      title: ['', [Validators.required, Validators.minLength(1), Validators.maxLength(50)]],
      questions: this.fb.array([]),  
      // answers: this.fb.array([])
    });

    setTimeout(() => {
      this.get_pipeline_item_by_id();
    }, 100);
  }

  // ===========================================================
  // Load questions
  async get_pipeline_item_by_id() {
    if (this.id == -1) { 
      this.toastr.error("Id is -1. Report error.", "Id must be larger or equal 0.");
      this.bsModalRef.dismiss("Id is -1. Please check bug.");
      return;
    }
    let row = {
      t_uuid: this.t_uuid,
      t_ver: this.t_ver,
      stage_index: this.curr_stage,
      pipeline_index: this.id
    }
    // console.warn(row);
    this.http3.send("/pipeline/proj", JSON.stringify(row)).then(async (value: any) => {
      let data = this.http3.json_handler(value);
      this.items = data;
      await this.loadData();
      // this.loading = false;  // call in loadData(). 
    }).catch(err => { this.doErr(err); this.loading = false; });
  }

  async loadData() {
    this.myForm.get('title')?.setValue(this.items.title);
    this.items.others.forEach((q: any) => {
      this.add_new_question(q);
    })
    this.loading = false;
  }

  add_new_question(data: any) {
    let qs = this.myForm.get('questions') as FormArray;
    qs.push(this.fb.group({
      question: [data.q], // a must
      q_type: [data.t],  // a must. 
      // NOTE: No need above like in template, because we don't need validator. 
      // On the other hand, the validator might be too complex if we do it not 
      // like how we do in template; but one never tried so one don't know. 
      // As for display, this is just fine, no need validators. 
      // But this will have problem, so solve using this: 
      // https://stackoverflow.com/questions/42322968/angular2-dynamic-input-field-lose-focus-when-input-changes
      rows: this.fb.array(data.r ?? []),
      cols: this.fb.array(data.c ?? []),
      min: [data.min ?? null],
      max: [data.max ?? null],
      min_name: [data.min_name ?? null],
      max_name: [data.max_name ?? null],

      // Only this is to be returned back. 
      answer: this.get_answer(data.t, data.r, data.c)
    }));
  }

  trackByFn(index: any, item: any) {
    return index;
  }

  // Return fb.array if is grid or checkbox; otherwise, return string with corresponding validators. 
  private get_answer(q_type: string, rows: any[], cols: any[]) {
    if (q_type == AnswerTypes.Checkbox) {
      return [[...Array(rows.length).fill(false)]]
    }
    if ([AnswerTypes.GridCheckbox, AnswerTypes.GridMultipleChoice].includes(q_type as AnswerTypes)) {
      return this.fb.array([])
    }
    return ['', this.get_validators(q_type)]
  }

  private get_validators(q_type: string) {
    // let qtype = parseInt(q_type);
    if (q_type == AnswerTypes.Short) { return [Validators.required, Validators.minLength(1), Validators.maxLength(75)] };
    if (q_type == AnswerTypes.Long) { return [Validators.required, Validators.minLength(1), Validators.maxLength(10_000)]};
    return [Validators.required];
  }

  // For checkbox only
  checking(index: number, event: any) {
    let checked = event.target.checked;
    console.log(`${checked} ${index}`);
    // TBD
  }

  // ===========================================================
  // Load answers

  // ===========================================================
  onSubmit() {

  }

  // ===========================================================
  get title() {
    return this.myForm.get('title')?.value;
  }

  get questions() {
    const q = this.myForm.get('questions') as FormArray;
    return q['controls'];
  }

  // get questions() {
  //   return this.myForm.get('questions')?.value;
  // }

  doErr(err: any) {
    console.error(err);
    this.toastr.error(err);
  }

  // ===========================================================
  // Helpers
  private get_formarray(first: string, i: number, second: string ): FormArray {
    let q = this.get_q(first, i);
    return q.get(second) as FormArray;
  }

  private get_q(first: string, i: number): AbstractControl {
    let qs = this.myForm.get(first) as FormArray;
    return qs.at(i);
  }
}
