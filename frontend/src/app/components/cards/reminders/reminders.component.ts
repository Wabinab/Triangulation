import { Component, Input, inject } from '@angular/core';
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

@Component({
  selector: 'app-reminders',
  standalone: true,
  imports: [SharedModule, SharedFormsModule, FontAwesomeModule, HumanPipe],
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

  items: any;
  loading: boolean = true;
  submitting: boolean = false;
  public myForm: FormGroup;

  constructor(private http3: Http3Service, private fb: FormBuilder, private translate: TranslateService) {
    this.myForm = this.fb.group({
      // id: [0, [Validators.required, Validators.min(1)]],  // ensure form invalid while loading.
      t: [CardTypes.Reminders, [Validators.required]],
      title: ['', [Validators.required, Validators.minLength(1), Validators.maxLength(50)]],
      questions: this.fb.array([])
    });
    this.add_new_question();

    setTimeout(() => { 
      this.get_pipeline_item_by_id();
      this.loading = false;
    }, 500);
  }

  // Remember to save clicking backdrop. 
  // Save is the default? User can choose in settings to turn this off. On is default. 
  async get_pipeline_item_by_id() {
    let data = {
      stage_step: this.curr_stage,
      pipeline_id: this.id,
      filename: this.filename
    }
    let value: any = await this.http3.send("/template/pipeline", JSON.stringify(data));
    // this.items = JSON.parse(value);
    await this.loadData(JSON.parse(value ?? '{}'));
  }

  async loadData(value: any) {
    console.log(value);
  }

  onSubmit() {
    this.submitting = true;
    const row = {
      // id: this.myForm.get('id')?.value,
      id: this.id,
      stage_step: this.curr_stage,
      filename: this.filename,
      locale: this.translate.currentLang ?? 'en',
      t: this.myForm.get('t')?.value,
      title: this.myForm.get('title')?.value,
      questions: this.filter_row()
    };

    console.log(row);

    // this.http3.send("/template/pipeline/reminder/save", JSON.stringify(row)).then((res: any) => {
    //   this.submitting = false;
    //   this.bsModalRef.close({ ty: res });
    // }, (error: any) => {
    //   console.error(error);
    //   this.submitting = false;
    // });
  }

  cancel() {
    this.bsModalRef.dismiss();
  }

  // Requires set locale here beforehand. 
  filter_row() {
    let qs = this.myForm.get('questions') as FormArray;
    let retval: any[] = [];
    qs.value.forEach((q: any) => {
      if (['2', '3'].includes(q.q_type)) {
        retval.push({
          q: q.question,
          t: q.q_type,
          r: q.rows.map((c: any) => c.option)
        });
      } else if (q.q_type == "4") {
        retval.push({
          q: q.question,
          t: q.q_type,
          min: q.min,
          max: q.max,
          min_name: q.min_name,
          max_name: q.max_name
        })
      } else if (['5', '6'].includes(q.q_type)) {
        retval.push({
          q: q.question,
          t: q.q_type,
          r: q.rows.map((c: any) => c.option),
          c: q.cols.map((c: any) => c.option)
        })
      } else {
        retval.push({
          q: q.question,
          t: q.q_type
        })
      }
    });
    return retval;
  }

  // ===========================================
  add_new_question() {
    let qs = this.myForm.get('questions') as FormArray;
    qs.push(this.fb.group({
      question: ['', [Validators.required, Validators.minLength(7), Validators.maxLength(255)]],
      q_type: [AnswerTypes.MultipleChoice, [Validators.required]],
      rows: this.fb.array([]),
      cols: this.fb.array([]),
      
      // For rating only (q_type = 4)
      min: [1, [Validators.required, Validators.min(0), Validators.max(1)]],
      max: [5, [Validators.required, Validators.min(2), Validators.max(10)]],
      min_name: ['', [Validators.required, Validators.minLength(1), Validators.maxLength(50)]],
      max_name: ['', [Validators.required, Validators.minLength(1), Validators.maxLength(50)]]
    }));
    this.on_qtype_change(qs.length - 1);
    this.add_rowcol(qs.length - 1, 0, 'rows');
    this.add_rowcol(qs.length - 1, 0, 'cols');
  }

  remove_question(i: number) {
    let qs = this.myForm.get('questions') as FormArray;
    if (qs.length == 1) return;
    qs.removeAt(i);
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
    // const q = this.get_q('questions', i);
  }



  // ===============================
  add_rowcol(i: number, j: number, rowcol = 'rows') {
    if (j > this.maxrowcol) return;
    let mcqs = this.get_formarray('questions', i, rowcol);
    mcqs.push(this.fb.group({
      option: ['', [Validators.required, Validators.minLength(10), Validators.maxLength(75)]]
    }));
  }

  remove_rowcol(i: number, j: number, rowcol = 'rows') {
    let mcqs = this.get_formarray('questions', i, rowcol);
    if (mcqs.length == 1) return;
    mcqs.removeAt(j);
  }

  clear_rowcol(i: number, rowcol = 'rows') {
    let mcqs = this.get_formarray('questions', i, rowcol);
    mcqs.clear();
  }

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
    console.log(i);
    const form_array = this.myForm.get('questions') as FormArray;
    console.warn(form_array.value.map((c: any) => c.question));
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
  }

  // private array_move(arr: any[], old_index: number, new_index: number) {
  //   if (new_index >= arr.length) {
  //       var k = new_index - arr.length + 1;
  //       while (k--) {
  //           arr.push(undefined);
  //       }
  //   }
  //   arr.splice(new_index, 0, arr.splice(old_index, 1)[0]);
  //   // return arr; // for testing
  // };

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
}
