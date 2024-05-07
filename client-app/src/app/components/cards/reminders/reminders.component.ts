import { Component, HostListener, Input, inject } from '@angular/core';
import { Http3Service } from '../../../services/http3.service';
import { SharedModule } from '../../../shared/shared.module';
import { SharedFormsModule } from '../../../shared/shared-forms.module';
import { AbstractControl, FormArray, FormBuilder, FormGroup, Validators } from '@angular/forms';
import { CardTypes } from '../../../models/card-types';
import { faAdd, faArrowsUpDown, faXmark } from '@fortawesome/free-solid-svg-icons';
import { AnswerTypes } from '../../../models/answer-types';
import { HumanPipe } from '../../../directives/human.pipe';
import { KeyValue } from '@angular/common';
import { faCircle, faSave, faSquare } from '@fortawesome/free-regular-svg-icons';
import { NgbActiveModal, NgbModal } from '@ng-bootstrap/ng-bootstrap';
import { MovetoComponent } from '../../moveto/moveto.component';
import { FontAwesomeModule } from '@fortawesome/angular-fontawesome';
import { TranslateService } from '@ngx-translate/core';
import { ToastrService } from 'ngx-toastr';
import { CancellationComponent } from '../../cancellation/cancellation.component';
import { Routes } from '../../../models/routes';
import { Subscription, interval } from 'rxjs';

@Component({
  selector: 'app-reminders',
  standalone: true,
  imports: [SharedModule, SharedFormsModule, FontAwesomeModule, 
    HumanPipe],
  templateUrl: './reminders.component.html',
  styleUrl: './reminders.component.scss'
})
export class RemindersComponent {
  bsModalRef = inject(NgbActiveModal);

  faAdd = faAdd;
  faCircle = faCircle;
  faSquare = faSquare;
  faCross = faXmark;
  faUpDown = faArrowsUpDown;
  faSave = faSave;

  originalOrder = (a: KeyValue<string,AnswerTypes>, b: KeyValue<string,AnswerTypes>): number => {
    return 0;
  }

  maxrowcol = 20;

  @Input() id: number = -1;
  @Input() curr_stage: number = 0;
  @Input() filename: string = '';

  AnswerTypes = AnswerTypes;

  is_new: boolean = true;
  items: any;
  loading: boolean = true;
  submitting: boolean = false;
  public myForm: FormGroup;
  subscription: Subscription;

  constructor(private http3: Http3Service, private fb: FormBuilder, 
    private translate: TranslateService, private toastr: ToastrService,
    // private cd: ChangeDetectorRef
  ) {
    this.myForm = this.fb.group({
      t: [CardTypes.Reminders, [Validators.required]],
      title: ['', [Validators.required, Validators.minLength(1), Validators.maxLength(50)]],
      questions: this.fb.array([])
    });
    // this.add_new_question();

    setTimeout(() => { 
      this.get_pipeline_item_by_id();
    }, 100);

    const source = interval(60_000 * 5);  // Save every 5 mins. 
    this.subscription = source.subscribe(_ => this.autoSave());
  }

  // Remember to save clicking backdrop. 
  // Save is the default? User can choose in settings to turn this off. On is default. 
  async get_pipeline_item_by_id() {
    let data = {
      filename: this.filename,
      stage_index: this.curr_stage,
      pipeline_index: this.id,
    }
    this.loading = true;
    let value: any = await this.http3.send(Routes.Pi, JSON.stringify(data));
    this.items = JSON.parse(value ?? '{}');
    if (this.items.err && this.items.err == "backend.OOBPipeline") {
      this.is_new = true;
      this.add_new_question();
      this.loading = false; return;
    }
    if (this.items.err && this.items.err.length > 0) { 
      this.doErr(this.items.err);
      this.loading = false; return; 
    }
    await this.loadData();
  }

  async loadData() {
    this.is_new = false;
    if (this.myForm.get('t')!.value != this.items.ty) { 
      this.doErr("reminder.MismatchCard"); return;
    }

    this.myForm.get('title')?.setValue(this.items.title);
    this.set_row();
    this.loading = false;
  }

  // ==============================================================================
  @HostListener("document:keydown", ['$event'])
  onSave(event: KeyboardEvent) {
    if (event.ctrlKey && event.key === 's') {
      event.preventDefault();
      this.autoSave("proj.ManualSave");
      // if (this.myForm.invalid) { this.doErr("err.InvalidForm"); return; }
    }
  }

  autoSave(msg = "proj.Autosave") {
    if (this.submitting || this.loading) return;
    this.toastr.info(this.translate.instant(msg), '', { timeOut: 1000 });
    this.submitting = true;
    const row = {
      filename: this.filename,
      stage_index: this.curr_stage,
      reminder_index: this.id,
      title: this.myForm.get('title')?.value,
      question: this.filter_row()
    }

    this.http3.send(this.is_new ? Routes.PiNew0 : Routes.PiEdit0, JSON.stringify(row))
    .then((res: any) => {
      this.submitting = false;
      this.is_new = false;
      // No need update reminder_index, as it's already defined even for new. 
    }).catch((err: any) => { this.doErr(err); this.submitting = false; });
  }

  onSubmit() {
    // if (this.myForm.invalid) { this.doErr("err.InvalidForm"); return; }
    if (this.submitting || this.loading) { this.wait(); return; }  // always invalid for unknown reason.
    this.submitting = true;
    const row = {
      filename: this.filename,
      stage_index: this.curr_stage,
      reminder_index: this.id,
      title: this.myForm.get('title')?.value,
      question: this.filter_row()
    }
    if (row.question === 'error') { this.submitting = false; return; } 

    this.http3.send(this.is_new ? Routes.PiNew0 : Routes.PiEdit0, JSON.stringify(row))
    .then((res: any) => {
      this.submitting = false;
      this.bsModalRef.close({ ty: this.http3.json_handler(res) });
    }).catch((err: any) => { this.doErr(err); this.submitting = false; });
  }

  modalCancel: any;
  cancel() {
    if (this.submitting || this.loading) { this.wait(); return; }
    // Too many actions that don't automatically
    // set dirty and touched. We'll just ask everytime, to be safe. 
    
    // if (this.is_dirty()) {
      this.modalCancel = this.modalSvc.open(CancellationComponent);
      this.modalCancel.componentInstance.back_path = "hide modal";
      this.modalCancel.componentInstance.back_dismiss = true;
      this.modalCancel.closed.subscribe((res: any) => {
        // yes, save (if valid)
        this.onSubmit();
        this.bsModalRef.dismiss();
      });
      this.modalCancel.dismissed.subscribe((_: any) => {
        this.bsModalRef.dismiss();
      })
    //   return;
    // }
    // this.bsModalRef.dismiss();
  }

  // private is_dirty() {
  //   let dirty = false;
  //   Object.keys(this.myForm.controls).forEach(key => {
  //     const field = this.myForm.get(key)!;
  //     if (field.dirty && field.touched) { dirty = true; }
  //   });
  //   return dirty;
  // }

  set_row() {
    this.items.others.forEach((q: any) => {
      this.add_new_question(q);
    })
  }

  filter_row() {
    let qs = this.myForm.get('questions') as FormArray;
    let retval: any[] = [];
    let error = false;
    qs.value.forEach((q: any, i: number) => {
      let invalid_controls: any = new Set(this.findInvalidControls(i));
      if (['2', '3'].includes(q.q_type)) {
        let set: any = new Set(['question', 'q_type', 'rows']);
        let intersection = invalid_controls.intersection(set);
        if (intersection.size > 0) { this.filter_row_err(i, intersection); error = true }
        retval.push({
          q: q.question,
          t: q.q_type,
          r: q.rows.map((c: any) => c.option)
        });
      } else if (q.q_type == "4") {
        let set: any = new Set(['question', 'q_type', 'min', 'max', 'min_name', 'max_name']);
        let intersection = invalid_controls.intersection(set);
        if (intersection.size > 0) { this.filter_row_err(i, intersection); error = true }
        retval.push({
          q: q.question,
          t: q.q_type,
          min: q.min,
          max: q.max,
          min_name: q.min_name,
          max_name: q.max_name
        })
      } else if (['5', '6'].includes(q.q_type)) {
        let set: any = new Set(['question', 'q_type', 'rows', 'cols']);
        let intersection = invalid_controls.intersection(set);
        if (intersection.size > 0) { this.filter_row_err(i, intersection); error = true }
        retval.push({
          q: q.question,
          t: q.q_type,
          r: q.rows.map((c: any) => c.option),
          c: q.cols.map((c: any) => c.option)
        })
      } else {
        let set: any = new Set(['question', 'q_type']);
        let intersection = invalid_controls.intersection(set);
        if (intersection.size > 0) { this.filter_row_err(i, intersection); error = true }
        retval.push({
          q: q.question,
          t: q.q_type
        })
      }
    });
    return error ? 'error' : retval;
  }

  private filter_row_err(i: number, set: any) {
    this.doErr("error.filter", {
      i: i + 1, value: JSON.stringify([...set])
    });
  }

  // ===========================================
  desc_limit = 10_000;
  readonly name_validators = [Validators.required, Validators.minLength(1), Validators.maxLength(50)];
  q_validators_1 = [Validators.required, Validators.minLength(7), Validators.maxLength(255)];
  q_validators_2 = [Validators.required, Validators.maxLength(this.desc_limit)];

  add_new_question(data: any = {}) {
    // if (this.submitting || this.loading) { this.wait(); return; }
    let qs = this.myForm.get('questions') as FormArray;
    let q_val = (data && data.t && data.t.toString() == '8') ? this.q_validators_2 : this.q_validators_1;
    qs.push(this.fb.group({
      question: [data.q ?? '', q_val],
      q_type: [data.t ?? AnswerTypes.Long, [Validators.required]],
      rows: this.fb.array([]),
      cols: this.fb.array([]),
      
      // For rating only (q_type = 4)
      min: [data.min ?? 1, [Validators.required, Validators.min(0), Validators.max(1)]],
      max: [data.max ?? 5, [Validators.required, Validators.min(2), Validators.max(10)]],
      min_name: [data.min_name ?? '', this.name_validators],
      max_name: [data.max_name ?? '', this.name_validators]
    }));
    if (!data.r) { this.add_rowcol(qs.length - 1, 0, 'rows'); }
    if (!data.c) this.add_rowcol(qs.length - 1, 0, 'cols');

    if (data.r) { data.r.forEach((c: string, i: number) => this.add_rowcol(qs.length - 1, i, 'rows', c)); }
    if (data.c) { data.c.forEach((c: string, i: number) => this.add_rowcol(qs.length - 1, i, 'cols', c)); }
  }

  remove_question(i: number) {
    if (this.submitting || this.loading) { this.wait(); return; }
    let qs = this.myForm.get('questions') as FormArray;
    if (qs.length == 1) return;
    qs.removeAt(i);
    qs.markAsDirty();
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

  on_qtype_change(i: number) {
    const q = this.get_q('questions', i);
    if (this.is_qtype(i, '8')) q.get('question')?.setValidators(this.q_validators_2);
    else q.get('question')?.setValidators(this.q_validators_1);

    q.updateValueAndValidity();
  }

  charcount: custom_option = {};
  textCounter(event: any, i: number) {
    const charcount = this.desc_limit - event.target.value.length;
    const translate_word = charcount >= 0 ? 'newTempl.charRemain' : 'newTempl.charOver';
    this.charcount[i.toString()] = `${Math.abs(charcount)} ${this.translate.instant(translate_word)}`;
  }

  // ===============================
  add_rowcol(i: number, j: number, rowcol = 'rows', data = '') {
    // if (this.submitting || this.loading) { this.wait(); return; }
    if (j > this.maxrowcol) return;
    let mcqs = this.get_formarray('questions', i, rowcol);
    mcqs.push(this.fb.group({
      option: [data, [Validators.required, Validators.minLength(1), Validators.maxLength(75)]]
    }));
  }

  remove_rowcol(i: number, j: number, rowcol = 'rows') {
    if (this.submitting || this.loading) { this.wait(); return; }
    let mcqs = this.get_formarray('questions', i, rowcol);
    if (mcqs.length == 1) return;
    mcqs.removeAt(j);
    mcqs.markAsDirty();
  }

  // clear_rowcol(i: number, rowcol = 'rows') {
  //   if (this.submitting || this.loading) { this.wait(); return; }
  //   let mcqs = this.get_formarray('questions', i, rowcol);
  //   mcqs.clear();
  // }

  rowcols(i: number, rowcol = 'rows') {
    const mcqs = this.get_formarray('questions', i, rowcol);
    return mcqs['controls'];
  }

  // ============================================
  // Modal
  private modalSvc = inject(NgbModal);

  modalMoveTo: any;
  // FormArray Level
  openModalUpDown(i: number, j: number, rowcol = 'rows') {
    const form_array = this.get_formarray('questions', i, rowcol);
    this.modalMoveTo = this.modalSvc.open(MovetoComponent);
    this.modalMoveTo.componentInstance.from = j + 1;  // j is zero based. 
    this.modalMoveTo.componentInstance.list_names = form_array.value.map((c: any) => c.option);
    this.modalMoveTo.closed.subscribe((res: any) => {
      this.array_move(form_array, j, res.ty);
    })
  }

  // Question Level
  openModalUpDownQLevel(i: number) {
    const form_array = this.myForm.get('questions') as FormArray;
    this.modalMoveTo = this.modalSvc.open(MovetoComponent);
    this.modalMoveTo.componentInstance.from = i + 1;  // i is zero-based.
    this.modalMoveTo.componentInstance.list_names = form_array.value.map((c: any) => c.question);
    this.modalMoveTo.closed.subscribe((res: any) => {
      this.array_move(form_array, i, res.ty);
    })
  }

  private array_move(arr: FormArray, old_index: number, new_index: number) {
    let item = arr.at(old_index);
    arr.removeAt(old_index);
    arr.insert(new_index, item);
    arr.markAsDirty();
  }

  // =============================================
  // Private functions and helps
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

  doErr(err: any, t_opt: any = {}) {
    console.error(err);
    if (typeof(err) === 'string') this.toastr.error(this.translate.instant(err || '', t_opt));
    else this.toastr.error(err);
  }

  wait() {
    this.toastr.info(this.translate.instant("wait"));
  }

  public findInvalidControls(i: number): any[] {
    const invalid: any[] = [];
    const mid_man: any = this.get_q('questions', i);
    const controls = mid_man.controls;
    for (const name in controls) {
      if (controls[name].invalid) invalid.push(name);
    }
    return invalid;
  }
}

type custom_option = {
  [key: string]: string
}