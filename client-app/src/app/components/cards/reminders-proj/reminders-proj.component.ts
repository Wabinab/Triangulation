import { Component, Input, inject } from '@angular/core';
import { SharedModule } from '../../../shared/shared.module';
import { SharedFormsModule } from '../../../shared/shared-forms.module';
import { FontAwesomeModule } from '@fortawesome/angular-fontawesome';
import { HumanPipe } from '../../../directives/human.pipe';
import { NgbActiveModal, NgbModal } from '@ng-bootstrap/ng-bootstrap';
import { AnswerTypes } from '../../../models/answer-types';
import { Http3Service } from '../../../services/http3.service';
import { AbstractControl, FormArray, FormBuilder, FormGroup, Validators } from '@angular/forms';
import { ToastrService } from 'ngx-toastr';
import { atLeastOneTrueValidator } from '../../../directives/at-least-one-true-validator.directive';
// import {MatDatepickerModule} from '@angular/material/datepicker';
import {MatInputModule} from '@angular/material/input';
import {MatFormFieldModule} from '@angular/material/form-field';
import {provideNativeDateAdapter} from '@angular/material/core';
import { TranslateService } from '@ngx-translate/core';
import { Subscription, interval } from 'rxjs';
import { CancellationComponent } from '../../cancellation/cancellation.component';
import { Routes } from '../../../models/routes';
// import { NgxMaterialTimepickerModule } from 'ngx-material-timepicker';

@Component({
  selector: 'app-reminders-proj',
  standalone: true,
  imports: [SharedModule, SharedFormsModule, FontAwesomeModule, HumanPipe,
    MatFormFieldModule, MatInputModule],
  providers: [provideNativeDateAdapter()],
  templateUrl: './reminders-proj.component.html',
  styleUrl: './reminders-proj.component.scss'
})
export class RemindersProjComponent {
  bsModalRef = inject(NgbActiveModal);
  private modalSvc = inject(NgbModal);

  @Input() id: number = -1;
  @Input() curr_stage: number = 0;
  @Input() filename: string = '';  // to save this file into project filename. 
  // For template filename. 
  @Input() t_uuid: string = '';
  @Input() t_ver: number = 0;

  AnswerTypes = AnswerTypes;
  desc_limit = 4_000;

  trackByFn(index: any, item: any) { return index; }

  items: any;
  loading: boolean = true;
  submitting: boolean = false;
  public myForm: FormGroup;
  subscription: Subscription;

  constructor(private http3: Http3Service, private fb: FormBuilder,
    private toastr: ToastrService, private translate: TranslateService
  ) {
    this.myForm = fb.group({
      title: [{value: '', disabled: true}, [Validators.required, Validators.minLength(1), Validators.maxLength(50)]],
      questions: this.fb.array([]),  
    });

    setTimeout(() => {
      this.get_pipeline_item_by_id();
    }, 100);

    // Save every 5 minute, if applicable. 
    const source = interval(60_000 * 5);
    // const source = interval(5_000);
    this.subscription = source.subscribe(_ => this.autoSave());
  }

  // ===========================================================
  // Load questions
  async get_pipeline_item_by_id() {
    if (this.id == -1) { 
      this.translate.get(["reminder.IdMinusOne", "reminder.IdMinusOneDesc"], {}).subscribe((res: any) => {
        this.toastr.error(res["reminder.IdMinusOne"], res["reminder.IdMinusOneDesc"]);
        this.bsModalRef.dismiss({ ty: res["reminder.IdMinusOne"] });
      }); return;
    }
    let row = {
      t_uuid: this.t_uuid,
      t_ver: this.t_ver,
      stage_index: this.curr_stage,
      pipeline_index: this.id
    }
    // console.warn(row);
    this.http3.send(Routes.PiProj, JSON.stringify(row)).then(async (value: any) => {
      let data = this.http3.json_handler(value);
      this.items = data;

      let row2 = {
        filename: this.filename,
        stage_index: this.curr_stage,
        pipeline_index: this.id
      };

      let answers = await this.http3.send(Routes.R, JSON.stringify(row2));
      let answers_json: any = this.http3.json_handler(answers);
      // Replace all '' with null, so it's easier to handle. 
      // Henceforth, answers SHOULD NEVER BE EMPTY STRING!!! 
      this.loadData(answers_json.map((c: any) => c === '' ? null : c));
      // this.loading = false;  // call in loadData(). 
    }).catch(err => { this.doErr(err); this.loading = false; });
  }

  loadData(answers_json: any[]) {
    this.myForm.get('title')?.setValue(this.items.title);
    this.items.others.forEach((q: any) => {
      this.add_new_question(q);
    });
    answers_json.forEach((a: any, i: number) => {
      this.fill_answers(a, i);
    });
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
      answer: this.get_answer_type(data.t, data.r, data.c)
    }));
  }

  // Use it AFTER add_new_questions; OTHERWISE WILL FAIL. 
  // The 'data' here isn't the same as 'data' in add_new_questions. 
  fill_answers(data: any, index: number) {
    let qs = this.myForm.get('questions') as FormArray;
    let q_type = qs.get([index, 'q_type'])!.value;
    let q = qs.get([index, 'answer'])!;
    if (data === null) return;  // return if null. 
    if (q_type === AnswerTypes.Checkbox) { q.setValue(this.index_to_bool(q.value ,data)); }
    else if (q_type === AnswerTypes.GridCheckbox) {
      let curr_2d_arr = q.value;
      data.map((indices_arr: number[], i: number) => {
        curr_2d_arr[i] = this.index_to_bool(curr_2d_arr[i], indices_arr);
      });
      q.setValue(curr_2d_arr);
    }
    else q.setValue(data);
  }

  // Return fb.array if is grid or checkbox; otherwise, return string with corresponding validators. 
  private get_answer_type(q_type: string, rows: any[], cols: any[]) {
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
    if (q_type == AnswerTypes.Long) { return [Validators.required, Validators.minLength(1), Validators.maxLength(this.desc_limit)]};
    return [Validators.required];
  }

  // For checkbox only
  checking(index: number, event: any, control: AbstractControl) {
    let value = control.value;
    value[index] = event.target.checked;
    control.setValue(value);
    this.mark_question_dirty_touched();
  }

  get_check_value(i: number, j: number) {
    let qs = this.myForm.get('questions') as FormArray;
    return qs.get([i, 'answer'])!.value[j];
  }

  // For grid checkbox only
  checking_grid(j: number, k: number, event: any, control: AbstractControl) {
    // let value = control.get([j, 'option'])!.value;
    let value = control.get([j])!.value;
    value[k] = event.target.checked;
    control.get([j])!.setValue(value);
    this.mark_question_dirty_touched();
  }

  get_check_value_grid(i: number, j: number, k: number) {
    let qs = this.myForm.get('questions') as FormArray;
    return qs.get([i, 'answer', j])!.value[k];
  }

  private mark_question_dirty_touched() {
    this.myForm.get("questions")!.markAsDirty();
    this.myForm.get("questions")!.markAsTouched();
  }

  // ===========================================================
  // Load answers

  // ===========================================================
  autoSave() {
    if (this.submitting || this.loading || this.myForm.invalid) return;
    this.translate.get('proj.Autosave', {}).subscribe((res: string) => {
      this.toastr.info(res, '', { timeOut: 1000 });
    });
    this.submitting = true;
    const row = {
      filename: this.filename,
      stage_index: this.curr_stage,
      pipeline_index: this.id,
      answer: this.get_answer()
    };

    this.http3.send(Routes.REdit, JSON.stringify(row)).then((_: any) => {
      this.submitting = false;
    }).catch(err => { this.doErr(err); this.submitting = false; })
  }

  onSubmit() {
    if (this.submitting || this.loading || this.myForm.invalid) return;
    this.submitting = true;
    const row = {
      filename: this.filename,
      stage_index: this.curr_stage,
      pipeline_index: this.id,
      answer: this.get_answer()
    };

    this.http3.send(Routes.REdit, JSON.stringify(row)).then((value: any) => {
      this.submitting = false;
      this.bsModalRef.close({ ty: this.http3.json_handler(value) });
    }).catch(err => { this.doErr(err); this.submitting = false; })
  }

  private get_answer() {
    let qs = this.myForm.get('questions')!.value;
    let answers: any[] = [];
    qs.forEach((q: any) => {
      if (q.q_type == AnswerTypes.Checkbox) { answers.push(this.bool_to_index(q.answer)); }
      else if (q.q_type == AnswerTypes.GridCheckbox) {
        answers.push(q.answer.map((c: boolean[]) => this.bool_to_index(c)));
      }
      else answers.push(q.answer);
    });
    return answers;
  }

  // ========================================================
  modalCancel: any;
  cancel() {
    if (this.loading || this.submitting) return;
    if (this.is_dirty()) {
      this.modalCancel = this.modalSvc.open(CancellationComponent);
      this.modalCancel.componentInstance.back_path = "hide modal";
      this.modalCancel.componentInstance.back_dismiss = true;
      this.modalCancel.closed.subscribe((res: any) => {
        // yes, save (if valid)
        this.onSubmit();
        this.bsModalRef.dismiss();
      });
      this.modalCancel.dismissed.subscribe((_: any) => this.bsModalRef.dismiss());
      return;
    }
    this.bsModalRef.dismiss();
  }

  clear_data() {
    if (this.submitting || this.loading) return;
    this.modalCancel = this.modalSvc.open(CancellationComponent);
    this.modalCancel.componentInstance.back_path = "hide modal";
    this.modalCancel.componentInstance.back_dismiss = true;
    this.modalCancel.closed.subscribe((res: any) => {
      this.submitting = true;
      const row = {
        filename: this.filename,
        stage_index: this.curr_stage,
        pipeline_index: this.id
      };
      this.http3.send(Routes.RDel, JSON.stringify(row)).then((value: any) => {
        this.http3.json_handler(value);
        this.translate.get('kelly.ClearData').subscribe((res: string) => {
          this.toastr.success(res);
        });
        this.submitting = false;
        this.get_pipeline_item_by_id();
      }).catch(err => { this.doErr(err); this.submitting = false; })
    });
  }

  private is_dirty() {
    let dirty = false;
    Object.keys(this.myForm.controls).forEach(key => {
      const field = this.myForm.get(key)!;
      if (field.dirty && field.touched) { dirty = true; }
    });
    return dirty;
  }

  // all_dirty(): string[] {
  //   let changedProperties: any[] = [];
  
  //   Object.keys(this.myForm.controls).forEach((name) => {
  //     const currentControl = this.myForm.controls[name];
  
  //     if (currentControl.dirty) {
  //       changedProperties.push(name);
  //     }
  //   });
  
  //   return changedProperties;
  // }

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

  // This doesn't work here, because we have multiple charcount. 
  charcount: string = '';
  textCounter(event: any, length: number | null = null) {
    const len = length ? length : event.target.value.length;
    const charcount = this.desc_limit - len;
    const translate_word = charcount >= 0 ? 'newTempl.charRemain' : 'newTempl.charOver';
    this.translate.get(translate_word, {}).subscribe((res: string) => {
      this.charcount = `${Math.abs(charcount)} ${res}`;
    });
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

  // private get_formarray(first: string, i: number, second: string ): FormArray {
  //   let q = this.get_q(first, i);
  //   return q.get(second) as FormArray;
  // }

  // private get_q(first: string, i: number): AbstractControl {
  //   let qs = this.myForm.get(first) as FormArray;
  //   return qs.at(i);
  // }

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
