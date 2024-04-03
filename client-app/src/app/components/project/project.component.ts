import { Component, HostListener, inject } from '@angular/core';
import { SharedModule } from '../../shared/shared.module';
import { FontAwesomeModule } from '@fortawesome/angular-fontawesome';
import { SharedFormsModule } from '../../shared/shared-forms.module';
import { DoubleClickDirective } from '../../directives/double-click.directive';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { HoverClassDirective } from '../../directives/hover-class.directive';
import { faPencil, faSave } from '@fortawesome/free-solid-svg-icons';
import { Http3Service } from '../../services/http3.service';
import { TranslateService } from '@ngx-translate/core';
import { ActivatedRoute } from '@angular/router';
import { FormBuilder } from '@angular/forms';
import { ToastrService } from 'ngx-toastr';
import { NgbModal } from '@ng-bootstrap/ng-bootstrap';
import { CancellationComponent } from '../cancellation/cancellation.component';
import { RemindersProjComponent } from '../cards/reminders-proj/reminders-proj.component';
import { Routes } from '../../models/routes';
import { KellyComponent } from '../cards/kelly/kelly.component';
import { KellyProjComponent } from '../cards/kelly-proj/kelly-proj.component';

@Component({
  selector: 'app-project',
  standalone: true,
  imports: [SharedModule, FontAwesomeModule, SharedFormsModule, DoubleClickDirective,
    MatProgressSpinnerModule, HoverClassDirective],
  templateUrl: './project.component.html',
  styleUrl: './project.component.scss'
})
export class ProjectComponent {
  faSave = faSave;
  faEdit = faPencil;

  stages: any[] = [];
  pipeline: any[] = [];
  filename = this.route.snapshot.queryParamMap.get('filename');
  project: any = {};
  template: any = {};
  loading: boolean = true;
  saving: boolean = false;
  newest_version = 1;

  constructor(private http3: Http3Service, public translate: TranslateService,
    private route: ActivatedRoute, private fb: FormBuilder, private toastr: ToastrService
  ) {
    this.loading = true;
    setTimeout(() => this.load(), 150);
  }

  async load(curr_stage: number = 0) {
    if (!this.filename) { 
      this.translate.get("proj.UndFilename", {}).subscribe((res: any) => {
        this.doErr(res);
      });
      this.loading = false; 
      return; 
    }
    const row = { filename: this.filename };

    // Any subsequent error can be catch this way; since it doesn't directly raise error but return Err json. 
    this.http3.send(Routes.P, JSON.stringify(row)).then(async (value: any) => {
      let data = this.http3.json_handler(value);
      this.project = data.project;
      this.template = data.template;
      this.stages = this.template?.stages;
      this.stages.sort(this.compareSteps);
      this.pipeline = this.stages[curr_stage]['pipeline'] ?? [];
  
      const row1 = { t_uuid: this.project.t_uuid };
      let value2: any = await this.http3.send(Routes.TVer, JSON.stringify(row1));
      let data2: any = this.http3.json_handler(value2);
      this.newest_version = data2.version;
  
      this.loading = false;
    }).catch(err => { this.doErr(err); this.loading = false; })
  }

  private compareSteps(a: any, b: any) {
    return a.step - b.step;
  }

  async save(save_ver: boolean = false) {
    this.saving = true;
    let row = {
      filename: this.filename,
      name: this.project['name'],
      description: this.project['description'],
      version: save_ver ? parseInt(this.project['t_ver']) : null
    };
    this.http3.send(Routes.PEdit, JSON.stringify(row)).then(data => {
      let _ = this.http3.json_handler(data);
      this.load();
      this.saving = false;
    }).catch(err => { this.doErr(err); this.load(); this.saving = false; });
  }

  /// Unsafe saving of version.
  async unsafe_save_ver() {
    this.saving = true;
    let row = {
      filename: this.filename,
      version: parseInt(this.project['t_ver'])
    };
    this.http3.send(Routes.PEditUnsafe, JSON.stringify(row)).then(data => {
      let _ = this.http3.json_handler(data);
      this.load();
      this.saving = false;
    }).catch(err => { this.doErr(err); this.load(); this.saving = false; });
  }

  // =====================================
  curr_stage: number = 0;

  sel_stage(value: number) {
    this.curr_stage = value;
    if (!this.stages[value]) { this.pipeline = []; return; }
    this.pipeline = this.stages[value]['pipeline'] ?? [];
  }

  is_active_stage(step: number) {
    return this.curr_stage == step ? 'nav-sidebar-active' : '';
  }

  // =====================================
  // Edit title and description. 
  title_name = '';
  is_edit_title = false;
  edit_title() {
    this.title_name = this.project.name;
    this.is_edit_title = true;
  }

  cancel_edit_title() {
    this.is_edit_title = false;
    this.title_name = this.project.name;
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
    this.project['name'] = this.title_name;
    this.is_edit_title = false;
    this.save();
  }

  desc_name = '';
  is_edit_desc = false;
  edit_desc() {
    this.desc_name = this.project.description;
    this.is_edit_desc = true;
  }

  cancel_edit_desc() {
    this.is_edit_desc = false;
    this.desc_name = this.project.description;
  }

  finish_edit_desc() {
    if (this.desc_name.length > 255) { 
      this.translate.get(["proj.AtMost", "proj.DescLong"], {value: 255}).subscribe((res: any) => {
        this.toastr.error(res["proj.AtMost"], res["proj.DescLong"]);
      }); return; 
    }
    this.project['description'] = this.desc_name;
    this.is_edit_desc = false;
    this.save();
  }

  curr_ver = 1;
  is_edit_ver = false;
  edit_ver() {
    this.curr_ver = this.project.t_ver;
    this.is_edit_ver = true;
  }

  cancel_edit_ver() {
    this.is_edit_ver = false;
    this.curr_ver = this.project.t_ver;
  }

  finish_edit_ver() {
    this.project['t_ver'] = this.curr_ver;
    this.is_edit_ver = false;
    this.save(true);
  }

  finish_edit_unsafe_ver() {
    this.project['t_ver'] = this.curr_ver;
    this.is_edit_ver = false;
    this.unsafe_save_ver();
  }

  get_versions(): number[] {
    return [...Array(this.newest_version + 1).keys()];
  }

  @HostListener('document:keydown.esc', ['$event'])
  esc_events(event: KeyboardEvent) {
    if (this.is_edit_title) this.cancel_edit_title();
    if (this.is_edit_desc) this.cancel_edit_desc();
    if (this.is_edit_ver) this.cancel_edit_ver();
  }

  // ====================================================
  private modalSvc = inject(NgbModal);

  modalCancel: any;
  confirm_version() {
    this.modalCancel = this.modalSvc.open(CancellationComponent);
    this.modalCancel.componentInstance.back_path = "hide modal";
    this.modalCancel.componentInstance.title = "proj.ConfirmVer";
    this.modalCancel.componentInstance.back_dismiss = true;
    this.modalCancel.closed.subscribe(async (_: any) => {
      this.finish_edit_ver();
    });
    this.modalCancel.dismissed.subscribe(async (_: any) => {
      this.is_edit_ver = false;
      this.translate.get("proj.VerChangeCancel").subscribe((res: any) => {
        this.toastr.warning(res);
      });
    });
  }

  confirm_unsafe_version() {
    this.modalCancel = this.modalSvc.open(CancellationComponent);
    this.modalCancel.componentInstance.back_path = "hide modal";
    this.modalCancel.componentInstance.title = "proj.ConfirmUnsafeVer";
    this.modalCancel.componentInstance.back_dismiss = true;
    this.modalCancel.closed.subscribe((_: any) => {
      this.finish_edit_unsafe_ver();
    });
    this.modalCancel.dismissed.subscribe((_: any) => {
      this.is_edit_ver = false;
      this.translate.get(["proj.VerChangeCancel", "proj.Unsafe"], {}).subscribe((res: any) => {
        this.toastr.warning(`${res["proj.Unsafe"]} ${res["proj.VerChangeCancel"]}`);
      });
    });
  }

  openTemplate(id: number, ty: number) {
    if (ty == 0) { this.openReminders(id, RemindersProjComponent); return; }
    if (ty == 1) { this.openReminders(id, KellyProjComponent, "lg"); return; }
  }

  modalReminder: any;
  openReminders(id: number, component: any = RemindersProjComponent, fullscreen = "sm") {
    this.modalReminder = this.modalSvc.open(component, {
      backdrop: 'static',
      fullscreen: fullscreen,
      size: 'xl'
    });
    this.modalReminder.componentInstance.id = id;
    this.modalReminder.componentInstance.curr_stage = this.curr_stage;
    this.modalReminder.componentInstance.filename = this.filename;
    this.modalReminder.componentInstance.t_uuid = this.project.t_uuid;
    this.modalReminder.componentInstance.t_ver = this.project.t_ver;
    this.modalReminder.closed.subscribe(async (_: any) => {
      await this.load(this.curr_stage)
    });
  }

  // ====================================================
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
    console.log(this.saving);
    console.error(err);
    this.toastr.error(err);
  }
}
