import { Component, Input } from '@angular/core';
import { Http3Service } from '../../../services/http3.service';
import { SharedModule } from '../../../shared/shared.module';
import { SharedFormsModule } from '../../../shared/shared-forms.module';
import { AbstractControl, FormArray, FormBuilder, FormGroup, Validators } from '@angular/forms';
import { CardTypes } from '../../../models/card-types';
import { faAdd, faXmark } from '@fortawesome/free-solid-svg-icons';
import { AnswerTypes } from '../../../models/answer-types';
import { HumanPipe } from '../../../directives/human.pipe';
import { KeyValue } from '@angular/common';
import { faCircle, faSquare } from '@fortawesome/free-regular-svg-icons';

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
    this.items = JSON.parse(value);
    await this.loadData(JSON.parse(value));
  }

  async loadData(value: any) {
    console.log(value);
  }

  onSubmit() {

  }

  // ===========================================
  new_question() {
    let qs = this.myForm.get('questions') as FormArray;
    qs.push(this.fb.group({
      question: ['', [Validators.required, Validators.minLength(10), Validators.maxLength(255)]],
      q_type: [AnswerTypes.MultipleChoice, [Validators.required]],
      rows: this.fb.array([]),
      columns: this.fb.array([])
    }));
    this.on_qtype_change(qs.length - 1);
  }

  get questions() {
    const q = this.myForm.get('questions') as FormArray;
    return q['controls'];
  }

  is_qtype(q: AbstractControl, value: string | string[]) {
    if (typeof value == 'object') return value.includes(q.get('q_type')!.value);
    return q.get('q_type')!.value == value;
  }

  on_qtype_change(i: number) {
    let qs = this.myForm.get('questions') as FormArray;
    let q = qs.at(i);

    // If MCQ or Checkbox
    if (['2', '3'].includes(q.get('q_type')!.value)) {
      if (q.get('rows')!.value.length == 0) this.add_row(i, 0);
    } else { this.clear_row(i); }
  }

  // ===============================
  add_row(i: number, j: number) {
    if (j > 20) { return; }
    let qs = this.myForm.get('questions') as FormArray;
    let q = qs.at(i);
    let mcqs = q.get('rows') as FormArray;
    mcqs.push(this.fb.group({
      option: [`Option ${j+1}`, [Validators.required, Validators.minLength(10), Validators.maxLength(75)]]
    }));
  }

  remove_row(i: number, j: number) {
    let qs = this.myForm.get('questions') as FormArray;
    let q = qs.at(i);
    let mcqs = q.get('rows') as FormArray;
    if (mcqs.length == 1) return;
    mcqs.removeAt(j);
  }

  clear_row(i: number) {
    let qs = this.myForm.get('questions') as FormArray;
    let q = qs.at(i);
    let mcqs = q.get('rows') as FormArray;
    mcqs.clear();
  }

  mcqs(i: number) {
    let qs = this.myForm.get('questions') as FormArray;
    let q = qs.at(i);
    const mcqs = q.get('rows') as FormArray;
    return mcqs['controls'];
  }
}
