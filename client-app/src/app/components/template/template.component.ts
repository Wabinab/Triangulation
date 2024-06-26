import { AfterViewInit, Component, ElementRef, HostListener, ViewChild, inject } from '@angular/core';
import { faBell, faListCheck, faMoneyBillWheat, faPencil, faPlus, faSave, faTrashAlt } from '@fortawesome/free-solid-svg-icons';
import { SharedModule } from '../../shared/shared.module';
import { SharedFormsModule } from '../../shared/shared-forms.module';
import { Http3Service } from '../../services/http3.service';
import { TranslateService } from '@ngx-translate/core';
import { NgbModal } from '@ng-bootstrap/ng-bootstrap';
import { RemindersComponent } from '../cards/reminders/reminders.component';
import { ActivatedRoute } from '@angular/router';
import { DoubleClickDirective } from '../../directives/double-click.directive';
// import { FormBuilder } from '@angular/forms';
import {MatProgressSpinnerModule} from '@angular/material/progress-spinner';
import { CancellationComponent } from '../cancellation/cancellation.component';
import { ToastrService } from 'ngx-toastr';
import { FontAwesomeModule } from '@fortawesome/angular-fontawesome';
import { HoverClassDirective } from '../../directives/hover-class.directive';
import { Routes } from '../../models/routes';
import { Tooltip } from 'bootstrap';
import { KellyComponent } from '../cards/kelly/kelly.component';
import { CardTypes } from '../../models/card-types';
import { ChecklistComponent } from '../cards/checklist/checklist.component';
import { HelperService } from '../../services/helper.service';

@Component({
  selector: 'app-template',
  standalone: true,
  imports: [SharedModule, FontAwesomeModule, SharedFormsModule, DoubleClickDirective, 
    MatProgressSpinnerModule, HoverClassDirective],
  templateUrl: './template.component.html',
  styleUrl: './template.component.scss'
})
export class TemplateComponent implements AfterViewInit {
  faSave = faSave;
  faAddStage = faPlus;
  faReminder = faBell;
  faEdit = faPencil;
  faRemove = faTrashAlt;
  faInvestment = faMoneyBillWheat;
  faChecklist = faListCheck;

  stages: any[] = [];
  filename = this.route.snapshot.queryParamMap.get('filename');
  template: any = {};
  loading: boolean = true;
  saving: boolean = false;

  @ViewChild('editStage') editStage: ElementRef;

  constructor(private http3: Http3Service, public translate: TranslateService,
    private route: ActivatedRoute, private toastr: ToastrService,
    public helperSvc: HelperService
  ) {
    this.loading = true;
    setTimeout(() => this.load(), 150);
  }

  ngAfterViewInit() {
    let tooltipTriggerList = [].slice.call(document.querySelectorAll('[data-bs-toggle="tooltip"]'))
    let _ = tooltipTriggerList.map(function (tooltipTriggerEl) {
      return new Tooltip(tooltipTriggerEl)
    });    
  }

  async load(curr_stage: number = 0) {
    if (!this.filename) { 
      this.doErr("proj.UndFilename");
      this.loading = false; return;
    }
    this.loading = true;
    const row = { filename: this.filename };
    this.http3.send(Routes.TNlist, JSON.stringify(row)).then(async (value: any) => {
      this.template = this.http3.json_handler(value);
      this.stages = this.template?.stages;
      this.stages.sort(this.compareSteps);
      this.pipeline = this.stages[curr_stage] ? this.stages[curr_stage]['pipeline'] : [];
      this.loading = false;
    }).catch(err => { this.doErr(err); this.loading = false; }) 
  }

  private compareSteps(a: any, b: any) {
    return a.step - b.step;
  }

  async save() {
    if (this.saving) { this.wait(); return; }
    this.saving = true;
    const row = {
      filename: this.filename,
      stages: this.stages,
      name: this.template.name,
      description: this.template.description,
    };
    this.http3.send(Routes.TEdit, JSON.stringify(row)).then((value: any) => {
      this.template = this.http3.json_handler(value);
      this.saving = false;
    }, (err: any) => { this.doErr(err); this.saving = false; });
  }

  // ===========================================
  // Sidebar
  curr_stage: number = 0;
  curr_edit_stage: number | null = null;
  initial_add_stage = false;
  pipeline: any = [];
  stage_name = '';  // ngModel field. 

  sel_stage(value: number) {
    if (this.curr_edit_stage != null && this.curr_edit_stage != value) this.finish_edit_stage();
    this._internal_sel_stage(value);
  }

  _internal_sel_stage(value: number) {
    this.curr_stage = value;
    if (!this.stages[value]) { this.pipeline = []; return; }
    this.pipeline = this.stages[value]['pipeline'] ?? [];
  }

  add_stage() {
    this.stages.push({ name: "", pipeline: [] });
    this.curr_stage = this.stages.length - 1;
    this.initial_add_stage = true;
    this.edit_stage(this.curr_stage);
  }

  edit_stage(stage: number) {
    this.curr_edit_stage = stage;
    this.stage_name = this.stages[stage]["name"];
    setTimeout((_: any) => {
      if (this.editStage !== undefined) this.editStage.nativeElement.focus();
    }, 100);
  }

  finish_edit_stage() {
    if (this.curr_edit_stage == null) return;
    this.stages[this.curr_edit_stage]['name'] = this.stage_name == '' 
      ? this.curr_edit_stage?.toString() : this.stage_name;
    this._internal_sel_stage(this.curr_edit_stage);
    this.curr_edit_stage = null;
    this.save();
  }

  delete_all_stages() {
    this.stages = [];
  }

  @HostListener('document:keydown.f2', ['$event'])
  keyboard_events(event: KeyboardEvent) {
    this.edit_stage(this.curr_stage);
  }

  @HostListener('document:keydown.esc', ['$event'])
  esc_events(event: KeyboardEvent) {
    if (this.curr_edit_stage !== null) this.finish_edit_stage();
    if (this.is_edit_title) this.cancel_edit_title();
    if (this.is_edit_desc) this.cancel_edit_desc();
  }

  // focusing = false;
  // ngAfterViewChecked(): void {
    // if (this.editStage !== undefined && !this.focusing) {
    //   this.editStage.nativeElement.focus();
    //   this.focusing = true;
    // }
  // }
  // @HostListener('document:click', ['$event'])
  // clickout(event: any) {
  //   if (!this.focusing) return;
  //   if (this.editStage.nativeElement.contains(event.target)) return;
  //   if (this.initial_add_stage) {
  //     this.initial_add_stage = !this.initial_add_stage;
  //     return;
  //   }
  //   this.finish_edit_stage();
  // }

  is_active_stage(step: number) {
    return this.curr_stage == step ? 'nav-sidebar-active' : '';
  }

  is_edit_stage(step: number) {
    return this.curr_edit_stage == step;
  }

  // =============================================
  // Edit title and description. 
  title_name = '';
  is_edit_title = false;
  edit_title() {
    this.title_name = this.template.name;
    this.is_edit_title = true;
  }

  cancel_edit_title() {
    this.is_edit_title = false;
    this.title_name = this.template.name;  // disable before set. 
  }

  finish_edit_title() {
    if (this.title_name.length < 1) { 
      this.translate.get(["proj.AtLeast", "proj.TitleShort"], {value: 1}).subscribe((res: any) => {
        this.toastr.error(res["proj.AtLeast"], res["proj.TitleShort"]);
      }); return;
    }
    if (this.title_name.length > 50) { 
      this.translate.get(["proj.AtMost", "proj.TitleLong"], {value: 50}).subscribe((res: any) => {
        this.toastr.error(res["proj.AtMost"], res["proj.TitleLong"]);
      }); return; 
    }
    this.template['name'] = this.title_name;
    this.is_edit_title = false;
    this.save();
  }

  desc_name = '';
  is_edit_desc = false;
  edit_desc() {
    this.desc_name = this.template.description;
    this.is_edit_desc = true;
  }

  cancel_edit_desc() {
    this.is_edit_desc = false;
    this.desc_name = this.template.description;
  }

  finish_edit_desc() {
    if (this.desc_name.length > 255) { 
      this.translate.get(["proj.AtMost", "proj.DescLong"], {value: 255}).subscribe((res: any) => {
        this.toastr.error(res["proj.AtMost"], res["proj.DescLong"]);
      }); return; 
    }
    this.template['description'] = this.desc_name;
    this.is_edit_desc = false;
    this.save();
  }

  // ================================================
  // Modals
  private modalSvc = inject(NgbModal);

  openTemplate(id: number, ty: number) {
    if (ty == CardTypes.Reminders) { this.openReminders(id); return; }
    if (ty == CardTypes.Kelly) { this.openKelly(id); return; }
    if (ty == CardTypes.Checklist) { this.openChecklist(id); return; }
  }

  modalReminder: any;
  openModal(id: number, component: any, options: any) {
    this.modalReminder = this.modalSvc.open(component, options);
    this.modalReminder.componentInstance.id = id;  // because slist won't return all items later on. 
    this.modalReminder.componentInstance.curr_stage = this.curr_stage;
    this.modalReminder.componentInstance.filename = this.filename;
    this.modalReminder.closed.subscribe(async (_: any) => {
      await this.load(this.curr_stage);
    });
  }

  openReminders(id: number) {
    this.openModal(id, RemindersComponent, {
      backdrop: 'static',
      fullscreen: 'sm',
      size: 'xl'
    });
  }
  new_reminder() {
    this.openReminders(this.pipeline.length);
  }

  openKelly(id: number) {
    this.openModal(id, KellyComponent, {
      backdrop: 'static',
      fullscreen: 'lg',
      size: 'xl'
    });
  }
  new_kelly() {
    this.openKelly(this.pipeline.length);
  }

  openChecklist(id: number) {
    this.openModal(id, ChecklistComponent, {
      backdrop: 'static',
      fullscreen: 'lg', 
      size: 'xl'
    });
  }
  new_checklist() {
    this.openChecklist(this.pipeline.length);
  }

  modalCancel: any;
  remove_stage() {
    // Need confirmation. 
    this.modalCancel = this.modalSvc.open(CancellationComponent);
    this.modalCancel.componentInstance.back_path = "hide modal";
    this.modalCancel.componentInstance.title = 'cancellation.Sure';
    this.modalCancel.componentInstance.back_dismiss = true;
    this.modalCancel.closed.subscribe(async (_: any) => {
      const i = this.curr_stage;
      if (i > -1) {
        var stage = this.stages.splice(i, 1)[0];
        this._internal_sel_stage(i - 1);
        this.toastr.success(this.translate.instant("templ.RemoveStage", {name: stage.name}));
        // this.translate.get("templ.RemoveStage", {name: stage.name}).subscribe((res: any) => {
        //   this.toastr.success(res);
        // }); 
        await this.save();
      }
    });
  }

  remove_question(i: number) {
    this.modalCancel = this.modalSvc.open(CancellationComponent);
    this.modalCancel.componentInstance.back_path = "hide modal";
    this.modalCancel.componentInstance.title = 'cancellation.Sure';
    this.modalCancel.componentInstance.back_dismiss = true;
    this.modalCancel.closed.subscribe(async (_: any) => {
      // Will call http3 later. 
      if (i > -1) {
        // var question = this.pipeline.splice(i, 1)[0];
        // this.toastr.success(`Removed Question ${i+1}: ${question.title}`);
        this.saving = true;
        const body = {
          filename: this.filename,
          stage_index: this.curr_stage,
          reminder_index: i
        };
        this.http3.send(Routes.PiDel0, JSON.stringify(body)).then(res => {
          console.log(this.http3.json_handler(res));
          var question = this.pipeline.splice(i, 1)[0];
          this.toastr.success(this.translate.instant("templ.RemoveQs", 
            {i: i+1, title: question.title}));
          // this.translate.get("templ.RemoveQs", {i: i+1, title: question.title})
          // .subscribe((res: any) => {
          //   this.toastr.success(res);
          // }); 
          this.saving = false;
        }).catch(err => { this.doErr(err); this.saving = false; });
      }
    });
  }

  // modalCancel: any;
  // cancel(): boolean {
  //   // if (this.loading || this.submitting) return;
  //   // if (this.is_dirty()) {
  //     this.modalCancel = this.modalSvc.open(CancellationComponent);
  //     this.modalCancel.componentInstance.back_path = "hide modal";
  //     const value = this.modalCancel.closed.subscribe((res: any) => {
  //       res["isClosed"] = true;
  //       console.log("cancellation closed");
  //       return true;
  //       // this.emitCallback.emit(res);
  //     });
  //     this.modalCancel.dismissed.subscribe((res: any) => {
  //       res["isClosed"] = false;
  //       console.log("cancellation dismissed");
  //       return false;
  //     });
  //     return value;
  // }


  // ================================================
  // To be moved to a service. 
  /// Get the translation of object. `obj` must have translation-readable
  /// keys. E.g. obj = {"en": "something", "fr": "quelque chose"}, the keys
  /// are "en" and "fr", which are translation-readable. 
  // get_translate(obj: any) {
  //   const keys = Object.keys(obj);
  //   const currentLang = this.translate.currentLang;
  //   if (keys.includes(currentLang)) return obj[currentLang];
  //   const browserLang = this.translate.getBrowserLang() ?? 'en';
  //   if (keys.includes(browserLang)) return obj[browserLang];
  //   const defaultLang = this.translate.getDefaultLang();
  //   if (keys.includes(defaultLang)) return obj[defaultLang];
  //   return obj[keys[0]];
  // }

  // get_locale(obj: any) {
  //   const currentLang = this.translate.currentLang;
  //   if (currentLang) return currentLang;
  //   const browserLang = this.translate.getBrowserLang();
  //   if (browserLang) return browserLang;
  //   const defaultLang = this.translate.getDefaultLang();
  //   if (defaultLang) return defaultLang;
  //   return Object.keys(obj)[0];
  // }

  doErr(err: any) {
    console.error(err);
    if (typeof(err) === 'string') this.toastr.error(this.translate.instant(err || ''));
    else this.toastr.error(err);
  }

  wait() {
    this.toastr.info(this.translate.instant("wait"));
  }

  disable_modals() {
    return { 'disable-modals': this.stages.length == 0 };
  }

  // test_get() {
  //   const row1 = {
  //     // filename: filename,
  //     name: this.template.name,
  //     description: this.template.description,
  //   };
  //   console.log(row1);
  // }
}
