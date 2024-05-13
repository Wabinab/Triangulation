import { Component, HostListener, Input, OnDestroy, inject } from '@angular/core';
import { SharedModule } from '../../../shared/shared.module';
import { SharedFormsModule } from '../../../shared/shared-forms.module';
import { FontAwesomeModule } from '@fortawesome/angular-fontawesome';
import { UppercaseDirective } from '../../../directives/uppercase.directive';
import { NgbActiveModal, NgbModal } from '@ng-bootstrap/ng-bootstrap';
import { faAdd, faCheck, faMinus, faTrashCan, faXmark } from '@fortawesome/free-solid-svg-icons';
import { faSave } from '@fortawesome/free-regular-svg-icons';
import { FormArray, FormBuilder, FormGroup, Validators } from '@angular/forms';
import { Subscription, interval } from 'rxjs';
import { Http3Service } from '../../../services/http3.service';
import { ToastrService } from 'ngx-toastr';
import { TranslateService } from '@ngx-translate/core';
import { Routes } from '../../../models/routes';
import { CancellationComponent } from '../../cancellation/cancellation.component';
import { DoubleClickDirective } from '../../../directives/double-click.directive';

@Component({
  selector: 'app-checklist-proj',
  standalone: true,
  imports: [SharedModule, SharedFormsModule, FontAwesomeModule, UppercaseDirective,
    DoubleClickDirective
  ],
  templateUrl: './checklist-proj.component.html',
  styleUrl: './checklist-proj.component.scss'
})
export class ChecklistProjComponent implements OnDestroy {
  bsModalRef = inject(NgbActiveModal);
  private modalSvc = inject(NgbModal);

  faAdd = faAdd;
  faCross = faXmark;
  faSave = faSave;
  faTrash = faTrashCan;
  faTick = faCheck;
  faMinus = faMinus;

  @Input() id: number = -1;
  @Input() curr_stage: number = 0;
  @Input() filename: string = "";
  @Input() t_uuid: string = '';
  @Input() t_ver: number = 0;

  items: any;
  loading: boolean = true;
  submitting: boolean = false;
  public myForm: FormGroup;
  subscription: Subscription;

  constructor(private http3: Http3Service, private fb: FormBuilder,
    private toastr: ToastrService, private translate: TranslateService
  ) {
    this.assign_initial_form();
    setTimeout(() => this.loadData(), 100);

    // Save every 5 minutes, if applicable. 
    const source = interval(300_000);
    this.subscription = source.subscribe(_ => this.autoSave());
  }

  ngOnDestroy(): void {
    this.subscription.unsubscribe();
  }

  private assign_initial_form() {
    this.myForm = this.fb.group({
      checklist: this.fb.array([]), 
      extra_checklist: this.fb.array([])
    });
  }

  // =============================================
  // Load questions
  async loadData() {
    if (this.id == -1) { 
      this.translate.get(["reminder.IdMinusOne", "reminder.IdMinusOneDesc"], {}).subscribe((res: any) => {
        this.toastr.error(res["reminder.IdMinusOne"], res["reminder.IdMinusOneDesc"]);
        this.bsModalRef.dismiss({ ty: res["reminder.IdMinusOne"] });
      }); return;
    }
    this.loading = true;
    let row = {
      t_uuid: this.t_uuid,
      t_ver: this.t_ver,
      stage_index: this.curr_stage,
      pipeline_index: this.id
    }
    this.http3.send(Routes.PiProj, JSON.stringify(row)).then(async (value: any) => {
      let data = this.http3.json_handler(value);
      this.items = data;

      let row2 = {
        filename: this.filename,
        stage_index: this.curr_stage,
        pipeline_index: this.id
      };

      let answers = await this.http3.send(Routes.RCL, JSON.stringify(row2));
      let answers_json: any = this.http3.json_handler(answers);
      this.fill_cycle(answers_json.map((c: any) => c.name));
      this.set_checklist(this.items.others, answers_json[this.cycle_id]);
      this.set_extra(answers_json[this.cycle_id]);
      this.loading = false;
    }).catch(err => { this.doErr(err); this.loading = false; });
  }

  async get_response() {
    let row2 = {
      filename: this.filename,
      stage_index: this.curr_stage,
      pipeline_index: this.id,
      cycle_index: this.cycle_id
    };

    this.loading = true;
    this.http3.send(Routes.RCL, JSON.stringify(row2)).then(async (answers: any) => {
      let answers_json: any = this.http3.json_handler(answers);
      // console.log(answers_json);
      this.set_checklist_again(answers_json);
      this.loading = false;
    }).catch((err: any) => { this.doErr(err); this.loading = false; });
  }

  private set_checklist(checklist: any, answers_json: any) {
    let c = this.myForm.get('checklist') as FormArray;

    if ([null, undefined].includes(checklist)) return; // nothing, just return. 
    checklist.forEach((d: string, i: number) => {
      c.push(this.fb.group({
        title: [{value: d ?? '', disabled: true}],
        check: [answers_json.data[i] ?? false]
      }));
    });
  }

  private set_extra(answers_json: any) {
    let e = this.myForm.get('extra_checklist') as FormArray;
    if ([null, undefined].includes(answers_json.extra)) return;
    answers_json.extra[0].forEach((val: any, i: number) => {
      e.push(this.fb.group({
        title: [val ?? '', [Validators.required, Validators.minLength(1), Validators.maxLength(1_000)]],
        check: [answers_json.extra[1][i] ?? false]
      }));
    })
  }

  private set_checklist_again(answers_json: any) {
    let c = this.myForm.get('checklist') as FormArray;

    let i = 0;
    for (let control of c.controls) {
      control.get('check')!.setValue(answers_json.data[i]);
      i++;
    }

    let e = this.myForm.get('extra_checklist') as FormArray;
    e.clear();
    this.set_extra(answers_json);
  }

  // ===============================================
  // Checklists
  add_to_list() {
    let c = this.myForm.get('extra_checklist') as FormArray;
    c.push(this.fb.group({
      title: ['', [Validators.required, Validators.minLength(1), Validators.maxLength(1_000)]],
      check: [false]
    }));
    return c.length;
  }

  add_and_focus(i: number | null = null, event: any = null) {
    if (this.loading || this.submitting) { this.wait(); return; }
    if (i !== null && (event.value.length === 0 || event.value === null)) return;
    let length = this.add_to_list();
    setTimeout(() => {
      document.getElementById(`ex_title_${i ?? length-1}`)!.focus();
    }, 10);
  }

  remove_item(i: number) {
    if (this.loading || this.submitting) { this.wait(); return; }
    let c = this.myForm.get('extra_checklist') as FormArray;
    c.removeAt(i);
    c.markAsDirty();
  }

  // ===============================================
  // Cycle Handler
  curr_edit_cycle = false;
  is_new_cycle = false;
  cycle_name = '';  // for edit template form. 
  cycle_id = 0;
  cycles = ["0"];
  max_cycle = 100;

  set_cycle(id: number) {
    this.cycle_id = id;
    this.get_response();
  }

  cycle_active(id: number) { return {'active': this.cycle_id == id }; }

  add_cycle() {
    if (this.loading || this.submitting) { this.wait(); return; }
    if (this.cycles.length >= this.max_cycle) { this.doErr("error.CycleMaxReached"); return; }
    this.cycle_id = this.cycles.length;
    // this.cycles.push(`Cycle ${this.cycles.length}`);
    this.cycles.push(`${this.cycles.length}`);
    this.is_new_cycle = true;
    this.edit_cycle_name(true);  // save after edit. 
  }

  remove_curr_cycle() {
    if (this.loading || this.submitting) { this.wait(); return; }
    if (this.cycles.length == 1) { this.doErr("error.OneCycle"); return; }

    let row2 = {
      filename: this.filename,
      stage_index: this.curr_stage,
      pipeline_index: this.id,
      cycle_index: this.cycle_id
    };
    this.submitting = true;
    this.http3.send(Routes.CDel, JSON.stringify(row2)).then((value: any) => {
      let _ = this.http3.json_handler(value);
      let name_arr = this.cycles.splice(this.cycle_id, 1);
      this.toastr.success(this.translate.instant("cycle.Remove", {value: name_arr.pop()}))
      this.cycle_id = 0;
      this.submitting = false;
      this.get_response();
    }).catch((err: any) => { this.doErr(err); this.submitting = false; });
  }

  // clear cycles in modal section.

  fill_cycle(json: any) {
    this.cycles = json.map((c: string) => c == '' ? 'NoName' : c);
  }

  @HostListener("document:keydown.f2", ['$event'])
  keyboard_events(event: KeyboardEvent) {
    this.edit_cycle_name();
  }

  @HostListener("document:keydown.esc", ['$event'])
  esc_events(event: KeyboardEvent) {
    if (this.curr_edit_cycle) { event.preventDefault(); this.finish_edit_cycle_name(); }
  }

  edit_cycle_name(select = false) {
    // if (this.loading || this.submitting) { this.wait(); return; }
    this.curr_edit_cycle = true;
    this.cycle_name = this.cycles[this.cycle_id];
    setTimeout(() => { 
      document.getElementById('cycle_name')?.focus() 
      if (select) (document.getElementById('cycle_name') as any)?.select();
    }, 10);
  }

  finish_edit_cycle_name() {
    this.curr_edit_cycle = false;
    this.cycles[this.cycle_id] = this.cycle_name;

    let row2 = {
      filename: this.filename,
      stage_index: this.curr_stage,
      pipeline_index: this.id,
      cycle_index: this.cycle_id,
      cycle_name: this.cycle_name
    };
    
    // save.
    this.submitting = true;
    this.http3.send(this.is_new_cycle ? Routes.CNew : Routes.CEdit, JSON.stringify(row2)).then((value: any) => {
      this.cycle_name = '';
      let _ = this.http3.json_handler(value);
      this.toastr.success(this.translate.instant("save", {value: row2.cycle_name}));
      this.submitting = false;
      this.is_new_cycle = false;
      this.get_response();
    }).catch((err: any) => { this.doErr(err); this.submitting = false; });
  }

  is_edit_cycle(id: number) {
    return this.curr_edit_cycle && this.cycle_id == id;
  }

  // ===============================================
  @HostListener("document:keydown", ['$event'])
  onSave(event: KeyboardEvent) {
    if (event.ctrlKey && event.key === 's') {
      event.preventDefault();
      this.autoSave("proj.ManualSave");
      if (this.myForm.invalid) { this.doErr("err.InvalidForm"); return; }
    }
  }
  
  autoSave(msg = "proj.Autosave") {
    if (this.submitting || this.loading || this.myForm.invalid) return; // no need invalid or wait. 
    this.toastr.info(this.translate.instant(msg, {}), '', { timeOut: 1000});
    this.submitting = true;
    const row = {
      filename: this.filename,
      stage_index: this.curr_stage,
      pipeline_index: this.id,
      cycle_index: this.cycle_id,
      checklist: this.get_answers(),
      extra_checklist: this.get_extra_answers()
    };
    this.http3.send(Routes.REditCL, JSON.stringify(row)).then((_: any) => {
      this.submitting = false; // Autosave no check for error. 
    }).catch(err => { this.doErr(err); this.submitting = false; });
  }

  onSubmit() {
    if (this.myForm.invalid) { this.doErr("err.InvalidForm"); return; }
    if (this.submitting || this.loading) { this.wait(); return; }
    this.submitting = true;
    const row = {
      filename: this.filename,
      stage_index: this.curr_stage,
      pipeline_index: this.id,
      cycle_index: this.cycle_id,
      checklist: this.get_answers(),
      extra_checklist: this.get_extra_answers()
    };
    this.http3.send(Routes.REditCL, JSON.stringify(row)).then((value: any) => {
      this.submitting = false;
      this.bsModalRef.close({ ty: this.http3.json_handler(value) });
    }).catch(err => { this.doErr(err); this.submitting = false; });
  }

  // checklist, only get tick/cross boolean. 
  // extra_checklist, requires both checklist_title and tick/cross boolean. 
  // Then, combine both together. 
  private get_answers() {
    let checklist = this.myForm.get('checklist')!.value;
    return checklist.map((c: any) => c.check === '' ? false : c.check);
  }

  private get_extra_answers() {
    let extra_checklist = this.myForm.get('extra_checklist')?.value;
    if (extra_checklist === null || extra_checklist.length == 0) return null;
    return [
      extra_checklist.map((c: any) => c.title),
      extra_checklist.map((c: any) => c.check === '' ? false : c.check)
    ];
  }

  // ==================================================
  // Modal
  modalCancel: any;
  cancel() {
    if (this.loading || this.submitting) return;
    if (this.is_dirty()) {
      this.modalCancel = this.modalSvc.open(CancellationComponent);
      this.modalCancel.componentInstance.back_path = "hide modal";
      this.modalCancel.componentInstance.back_dismiss = true;
      this.modalCancel.closed.subscribe((_: any) => {
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
    if (this.loading || this.submitting) { this.wait(); return; }
    this.modalCancel = this.modalSvc.open(CancellationComponent);
    this.modalCancel.componentInstance.back_path = "hide modal";
    this.modalCancel.componentInstance.back_dismiss = true;
    this.modalCancel.componentInstance.title = 'cancellation.Sure';
    this.modalCancel.closed.subscribe((res: any) => {
      this.submitting = true;
      const row = {
        filename: this.filename,
        stage_index: this.curr_stage,
        pipeline_index: this.id,
        cycle_index: this.cycle_id
      };
      this.http3.send(Routes.RDelCL, JSON.stringify(row)).then((value: any) => {
        this.http3.json_handler(value);
        this.toastr.success(this.translate.instant('kelly.ClearData'));
        this.submitting = false;
        this.reset_form();
      }).catch(err => { this.doErr(err); this.submitting = false; })
    });
  }

  clear_cycles() {
    if (this.loading || this.submitting) { this.wait(); return; }
    this.modalCancel = this.modalSvc.open(CancellationComponent);
    this.modalCancel.componentInstance.back_path = "hide modal";
    this.modalCancel.componentInstance.back_dismiss = true;
    this.modalCancel.componentInstance.title = 'cancellation.Sure';
    this.modalCancel.closed.subscribe((res: any) => {
      const first = this.cycles[0];
      this.cycles = [first];
      let row2 = {
        filename: this.filename,
        stage_index: this.curr_stage,
        pipeline_index: this.id
      };
      this.submitting = true;
      this.http3.send(Routes.CClear, JSON.stringify(row2)).then((value: any) => {
        let _ = this.http3.json_handler(value);
        this.toastr.success(this.translate.instant("cycle.ClearSuccess", {value: ''}));
        this.cycle_id = 0;
        this.submitting = false;
        this.get_response();
      }).catch((err: any) => { this.doErr(err); this.submitting = false; });
    });
  }

  private reset_form() {
    this.assign_initial_form();
    this.loadData();
  }
  
  private is_dirty() {
    let dirty = false;
    Object.keys(this.myForm.controls).forEach(key => {
      const field = this.myForm.get(key)!;
      if (field.dirty && field.touched) { dirty = true; }
    });
    return dirty;
  }

  // ==================================================
  // Helpers
  get title() { return this.items?.title ?? 'Untitied'; }
  get checklists() {
    const q = this.myForm.get('checklist') as FormArray;
    return q['controls'];
  }
  get extra_checklists() {
    const q = this.myForm.get('extra_checklist') as FormArray;
    return q['controls'];
  }

  doErr(err: any) {
    console.error(err);
    if (typeof(err) === 'string') this.toastr.error(this.translate.instant(err || ''));
    else this.toastr.error(err);
  }

  wait() {
    this.toastr.info(this.translate.instant("wait"));
  }

  // ============================
  // Debug
  // set_loading() {
  //   this.loading = true;
  // }
}
