import { AfterViewChecked, Component, ElementRef, HostListener, ViewChild, inject } from '@angular/core';
import { faBell, faPencil, faPlus, faSave } from '@fortawesome/free-solid-svg-icons';
import { SharedModule } from '../../shared/shared.module';
import { SharedFormsModule } from '../../shared/shared-forms.module';
import { Http3Service } from '../../services/http3.service';
import { TranslateService } from '@ngx-translate/core';
import { NgbModal } from '@ng-bootstrap/ng-bootstrap';
import { RemindersComponent } from '../cards/reminders/reminders.component';
import { ActivatedRoute } from '@angular/router';
import { DoubleClickDirective } from '../../directives/double-click.directive';
import { FormBuilder } from '@angular/forms';
import {MatProgressSpinnerModule} from '@angular/material/progress-spinner';
import { CancellationComponent } from '../cancellation/cancellation.component';
import { ToastrService } from 'ngx-toastr';
import { FontAwesomeModule } from '@fortawesome/angular-fontawesome';

@Component({
  selector: 'app-template',
  standalone: true,
  imports: [SharedModule, FontAwesomeModule, SharedFormsModule, DoubleClickDirective, MatProgressSpinnerModule],
  templateUrl: './template.component.html',
  styleUrl: './template.component.scss'
})
export class TemplateComponent {
  faSave = faSave;
  faAddStage = faPlus;
  faReminder = faBell;
  faEdit = faPencil;

  stages: any[] = [];
  filename = this.route.snapshot.queryParamMap.get('filename');
  template: any = {};
  loading: boolean = true;
  saving: boolean = false;

  @ViewChild('editStage') editStage: ElementRef;

  constructor(private http3: Http3Service, public translate: TranslateService,
    private route: ActivatedRoute, private fb: FormBuilder, private toastr: ToastrService
  ) {
    this.loading = true;
    setTimeout(() => this.load(), 500);
  }

  async load() {
    if (!this.filename) { this.doErr("Filename not defined.", this.loading); return;}
    const row = {
      filename: this.filename
    };
    let value: any = await this.http3.send("/template/nlist", JSON.stringify(row));
    this.template = JSON.parse(value);
    this.stages = this.template?.stages;
    this.stages.sort(this.compareSteps);
    this.loading = false;
  }

  private compareSteps(a: any, b: any) {
    return a.step - b.step;
  }

  async save() {
    this.saving = true;
    const row = {
      filename: this.filename,
      stages: this.stages,
      name: this.template.name,
      description: this.template.description,
    };
    this.http3.send("/template/edit", JSON.stringify(row)).then((value: any) => {
      this.template = JSON.parse(value);
      this.saving = false;
    }, (err: any) => { this.doErr(err, this.saving); });
  }

  // ===========================================
  // Sidebar
  curr_stage: number = 0;
  curr_edit_stage: number | null = null;
  initial_add_stage = false;
  pipeline: any = [];
  stage_name = '';  // ngModel field. 

  sel_stage(value: number) {
    this.curr_stage = value;
    if (this.curr_edit_stage != null && this.curr_edit_stage != value) this.finish_edit_stage();
    this.pipeline = this.stages[value]['pipeline'] ?? [];
  }

  add_stage() {
    this.stages.push({ name: "", pipeline: [] });
    this.curr_stage = this.stages.length;
    this.initial_add_stage = true;
    this.edit_stage(this.curr_stage - 1);
  }

  edit_stage(stage: number) {
    this.curr_edit_stage = stage;
    this.stage_name = this.stages[stage]["name"];
  }

  finish_edit_stage() {
    if (this.curr_edit_stage == null) return;
    this.stages[this.curr_edit_stage]['name'] = this.stage_name == '' 
      ? this.curr_edit_stage?.toString() : this.stage_name;
    this.curr_edit_stage = null;
  }

  delete_all_stages() {
    this.stages = [];
  }

  @HostListener('document:keydown.f2', ['$event'])
  keyboard_events(event: KeyboardEvent) {
    this.edit_stage(this.curr_stage);
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

  finish_edit_title() {
    if (this.title_name.length < 1) { this.toastr.error('At least 1 character.', "Title too short"); return; }
    if (this.title_name.length > 50) { this.toastr.error('At most 50 characters.', 'Title too long'); return; }
    this.template['name'] = this.title_name;
    this.is_edit_title = false;
  }

  desc_name = '';
  is_edit_desc = false;
  edit_desc() {
    this.desc_name = this.template.description;
    this.is_edit_desc = true;
  }

  finish_edit_desc() {
    if (this.desc_name.length > 255) { this.toastr.error('At most 255 characters.', 'Description too long.'); return; }
    this.template['description'] = this.desc_name;
    this.is_edit_desc = false;
  }

  // ================================================
  // Modals
  private modalSvc = inject(NgbModal);

  modalReminder: any;
  openReminders(id: number) {
    this.modalReminder = this.modalSvc.open(RemindersComponent, {
      backdrop: 'static',
      fullscreen: 'sm',
      size: 'xl'
    });
    // this.modalReminder.componentInstance = {id: 0};  // this doesn't work. 
    this.modalReminder.componentInstance.id = id;  // because slist won't return all items later on. 
    this.modalReminder.componentInstance.curr_stage = this.curr_stage;
    this.modalReminder.componentInstance.filename = this.filename;
    this.modalReminder.closed.subscribe((res: any) => {
      console.log("closed");
    });
    // this.modalReminder.dismissed.subscribe((res: any) => {
    //   console.log("dismissed");
    // });
  }

  new_reminder() {
    const no_of_pipeline = this.pipeline.length;
    this.openReminders(no_of_pipeline + 1);
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
  get_translate(obj: any) {
    const keys = Object.keys(obj);
    const currentLang = this.translate.currentLang;
    if (keys.includes(currentLang)) return obj[currentLang];
    const browserLang = this.translate.getBrowserLang() ?? 'en';
    if (keys.includes(browserLang)) return obj[browserLang];
    const defaultLang = this.translate.getDefaultLang();
    if (keys.includes(defaultLang)) return obj[defaultLang];
    return obj[keys[0]];
  }

  get_locale(obj: any) {
    const currentLang = this.translate.currentLang;
    if (currentLang) return currentLang;
    const browserLang = this.translate.getBrowserLang();
    if (browserLang) return browserLang;
    const defaultLang = this.translate.getDefaultLang();
    if (defaultLang) return defaultLang;
    return Object.keys(obj)[0];
  }

  doErr(err: any, set_false: boolean) {
    set_false = false;
    console.error(err);
    this.toastr.error(err);
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
