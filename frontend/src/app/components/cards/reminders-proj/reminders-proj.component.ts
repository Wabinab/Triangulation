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
import { atLeastOneTrueValidator } from '../../../directives/at-least-one-true-validator.directive';
import {MatDatepickerModule} from '@angular/material/datepicker';
import {MatInputModule} from '@angular/material/input';
import {MatFormFieldModule} from '@angular/material/form-field';
import {provideNativeDateAdapter} from '@angular/material/core';
// import { NgxMaterialTimepickerModule } from 'ngx-material-timepicker';

@Component({
  selector: 'app-reminders-proj',
  standalone: true,
  imports: [SharedModule, SharedFormsModule, FontAwesomeModule, HumanPipe,
    MatFormFieldModule, MatInputModule, MatDatepickerModule],
  providers: [provideNativeDateAdapter()],
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
      min: [parseInt(data.min) ?? null],
      max: [parseInt(data.max) ?? null],
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
      return [[...Array(rows.length).fill(false)], atLeastOneTrueValidator()] // We'll think of validators later for at least one true. 
    }
    if ([AnswerTypes.GridCheckbox, AnswerTypes.GridMultipleChoice].includes(q_type as AnswerTypes)) {
      let arr: FormArray = this.fb.array([]);
      for (let r in rows) {
        // arr.push(this.fb.group({
        //   option: q_type == AnswerTypes.GridCheckbox 
        //     ? this.get_answer(AnswerTypes.Checkbox, cols, []) // loop over cols instead of rows
        //     : ['', Validators.required]
        // }));
        arr.push(q_type == AnswerTypes.GridCheckbox
          ? this.fb.control([...Array(cols.length).fill(false)], [atLeastOneTrueValidator()])
          : this.fb.control('', [Validators.required])  
        );
      }
      return arr;
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
  checking(index: number, event: any, control: AbstractControl) {
    let value = control.value;
    value[index] = event.target.checked;
    control.setValue(value);
  }

  // For grid checkbox only
  checking_grid(j: number, k: number, event: any, control: AbstractControl) {
    // let value = control.get([j, 'option'])!.value;
    let value = control.get([j])!.value;
    value[k] = event.target.checked;
    control.get([j, 'option'])!.setValue(value);
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
  // This is inclusive of min and max. 
  range_min_max(min: number, max: number) : ReadonlyArray<number> {
    const size = Math.abs(max - min + 1);
    const startAt = min;
    return this.range(size, startAt);
  }

  range(size:number, startAt:number = 0) : ReadonlyArray<number> {
    return [...Array(size).keys()].map(i => i + startAt);
  }

  at_least_one_true_css(errors: any): string {
    if (errors && errors.atLeastOneTrue) { return "ng-invalid" }
    return "ng-valid";
  }

  private get_formarray(first: string, i: number, second: string ): FormArray {
    let q = this.get_q(first, i);
    return q.get(second) as FormArray;
  }

  private get_q(first: string, i: number): AbstractControl {
    let qs = this.myForm.get(first) as FormArray;
    return qs.at(i);
  }

  // Convert boolean array to indices. 
  // https://stackoverflow.com/questions/50981806/javascript-get-indices-of-true-values-in-a-boolean-array
  private bool_to_index(arr: any[]) {
    return arr.reduce(
      (out: any, bool: boolean, index: number) => bool ? out.concat(index) : out, 
      []
    );
  }

  private index_to_bool(bool_arr: any[], indices_arr: number[]) {
    indices_arr.map(i => bool_arr[i] = true);
    return bool_arr;
  }
}
