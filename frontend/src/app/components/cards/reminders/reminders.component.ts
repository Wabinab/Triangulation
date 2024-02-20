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
import { faCircle, faSquare } from '@fortawesome/free-regular-svg-icons';
import { NgbModal } from '@ng-bootstrap/ng-bootstrap';
import { MovetoComponent } from '../../moveto/moveto.component';

@Component({
  selector: 'app-reminders',
  standalone: true,
  imports: [SharedModule, SharedFormsModule, HumanPipe],
  templateUrl: './reminders.component.html',
  styleUrl: './reminders.component.scss'
})
export class RemindersComponent {
  faAdd = faAdd;
  faCircle = faCircle;
  faSquare = faSquare;
  faCross = faXmark;
  faUpDown = faArrowsUpDown;

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
  public myForm: FormGroup;

  constructor(private http3: Http3Service, private fb: FormBuilder) {
    this.myForm = this.fb.group({
      id: [this.id, [Validators.required, Validators.min(1)]],  // ensure form invalid while loading.
      t: [CardTypes.Reminders, [Validators.required]],
      title: ['', [Validators.required, Validators.minLength(1), Validators.maxLength(50)]],
      questions: this.fb.array([])
    });

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

  }

  // ===========================================
  add_new_question() {
    let qs = this.myForm.get('questions') as FormArray;
    qs.push(this.fb.group({
      question: ['', [Validators.required, Validators.minLength(10), Validators.maxLength(255)]],
      q_type: [AnswerTypes.MultipleChoice, [Validators.required]],
      rows: this.fb.array([]),
      cols: this.fb.array([])
    }));
    this.on_qtype_change(qs.length - 1);
    this.add_rowcol(qs.length - 1, 0, 'rows');
    this.add_rowcol(qs.length - 1, 0, 'cols');
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

    // If MCQ or Checkbox
    // if (['2', '3', '5', '6'].includes(q.get('q_type')!.value)) {
    //   if (q.get('rows')!.value.length == 0) this.add_rowcol(i, 0);
    // } else { this.clear_rowcol(i); }

    // if (['5', '6'].includes(q.get('q_type')!.value)) {
    //   if (q.get('cols')!.value.length == 0) this.add_rowcol(i, 0, 'cols');
    // } else { this.clear_rowcol(i, 'cols'); }
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
  openModalUpDown(i: number, j: number, rowcol = 'rows') {
    const form_array = this.get_formarray('questions', i, rowcol);
    this.modalMoveTo = this.modalSvc.open(MovetoComponent);
    this.modalMoveTo.componentInstance.from = j + 1;  // j is zero based. 
    this.modalMoveTo.componentInstance.list_names = form_array.value.map((c: any) => c.option);
    this.modalMoveTo.closed.subscribe((res: any) => {
      this.array_move(form_array, j, res.ty);
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
  // Private functions
  private get_formarray(first: string, i: number, second: string ): FormArray {
    let q = this.get_q(first, i);
    return q.get(second) as FormArray;
  }

  private get_q(first: string, i: number): AbstractControl {
    let qs = this.myForm.get(first) as FormArray;
    return qs.at(i);
  }
}
