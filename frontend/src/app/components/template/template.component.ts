import { AfterViewChecked, Component, ElementRef, HostListener, ViewChild, inject } from '@angular/core';
import { faPlus, faSave } from '@fortawesome/free-solid-svg-icons';
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

@Component({
  selector: 'app-template',
  standalone: true,
  imports: [SharedModule, SharedFormsModule, DoubleClickDirective, MatProgressSpinnerModule],
  templateUrl: './template.component.html',
  styleUrl: './template.component.scss'
})
export class TemplateComponent implements AfterViewChecked {
  faSave = faSave;
  faAddStage = faPlus;
  stages: any[] = [];
  template: any = {};
  loading: boolean = true;
  saving: boolean = false;

  @ViewChild('editStage') editStage: ElementRef;

  constructor(private http3: Http3Service, public translate: TranslateService,
    private route: ActivatedRoute, private fb: FormBuilder
  ) {
    // this.get_fivestep();
    this.loading = true;
    setTimeout(() => this.load(), 500);
  }

  async load() {
    let filename = this.route.snapshot.queryParamMap.get('filename');
    if (filename) {
      const row = {
        filename: filename
      };
      let value: any = await this.http3.send("/template", JSON.stringify(row));
      // console.log(value);
      this.template = JSON.parse(value);
      this.stages = this.template?.stages;
      this.stages.sort(this.compareSteps);
      this.loading = false;
      return;
    }
    // Replace with doErr later. 
    console.error("Cannot find file.");
    this.loading = false;
  }

  save() {
    this.saving = true;
  }

  // ===========================================
  // Sidebar
  curr_stage: number = 0;
  curr_edit_stage: number | null = null;
  pipeline: any = {};
  sel_stage(value: number) {
    this.curr_stage = value;
    // this.curr_edit_stage = null;
    if (this.curr_edit_stage != value) this.finish_edit_stage();
    this.pipeline = this.stages.find(c => c.step == value)['pipeline'] ?? [];
  }

  initial_add_stage = false;
  add_stage() {
    const lang = this.translate.getBrowserLang() ?? 'en';
    let item: any = { step: this.stages.length + 1, name: {}, pipeline: [] };
    // let name: any = {};
    item["name"][lang] = "";
    this.stages.push(item);
    
    this.curr_stage = this.stages.length;
    this.initial_add_stage = true;
    this.edit_stage(this.curr_stage);
  }

  stage_name = '';
  edit_stage(stage: number) {
    this.curr_edit_stage = stage;
    this.stage_name = this.get_translate(this.stages.find(c => c.step == stage)['name'] ?? '');
    // console.log("edit stage ", stage);
  }

  finish_edit_stage() {
    const lang = this.get_language(this.stages.find(c => c.step == this.curr_edit_stage)['name']);
    this.stages.find(c => c.step == this.curr_edit_stage)['name'][lang] = this.stage_name == '' 
      ? this.curr_edit_stage?.toString() : this.stage_name;
    this.curr_edit_stage = null;
    // Sent to backend to save. TBD. 
  }

  focusing = false;
  ngAfterViewChecked(): void {
    if (this.editStage !== undefined && !this.focusing) {
      this.editStage.nativeElement.focus();
      this.focusing = true;
    }
  }
  @HostListener('document:click', ['$event'])
  clickout(event: any) {
    if (!this.focusing) return;
    if (this.editStage.nativeElement.contains(event.target)) return;
    if (this.initial_add_stage) {
      this.initial_add_stage = !this.initial_add_stage;
      return;
    }
    this.finish_edit_stage();
  }

  is_active_stage(step: number) {
    return this.curr_stage == step ? 'nav-sidebar-active' : '';
  }

  is_edit_stage(step: number) {
    return this.curr_edit_stage == step;
  }

  // =============================================
  // Debug sample five step ray dalio
  // async get_fivestep() {
  //   let value: any = await this.http3.send("/sample_template", "/sample_templ/five_step_ray_dalio.json");
  //   // let value = await this.http3.send("/", "this confirm will work");
  //   this.template = JSON.parse(value);
  //   this.stages = this.template?.stages;
  //   this.stages.sort(this.compareSteps);
  // }

  private compareSteps(a: any, b: any) {
    return a.step - b.step;
  }

  // ================================================
  // Modals
  private modalSvc = inject(NgbModal);

  modalReminder: any;
  openReminders(id: number) {
    this.modalReminder = this.modalSvc.open(RemindersComponent);
    // this.modalReminder.componentInstance = {id: 0};  // this doesn't work. 
    this.modalReminder.componentInstance.id = id;  // because slist won't return all items later on. 
    this.modalReminder.componentInstance.curr_stage = this.curr_stage;
    this.modalReminder.componentInstance.filename = "/sample_templ/five_step_ray_dalio.json";
    this.modalReminder.closed.subscribe((res: any) => {});
    this.modalReminder.dismissed.subscribe((res: any) => {});
  }


  // ================================================
  // To be moved to a service. 
  /// Get the translation of object. `obj` must have translation-readable
  /// keys. E.g. obj = {"en": "something", "fr": "quelque chose"}, the keys
  /// are "en" and "fr", which are translation-readable. 
  get_translate(obj: any) {
    const keys = Object.keys(obj);
    const browserLang = this.translate.getBrowserLang() ?? 'en';
    if (keys.includes(browserLang)) return obj[browserLang];
    const defaultLang = this.translate.getDefaultLang();
    if (keys.includes(defaultLang)) return obj[defaultLang];
    // If none, return the first among the keys. 
    return obj[keys[0]];
  }

  get_language(obj: any) {
    const browserLang = this.translate.getBrowserLang();
    if (browserLang) return browserLang;
    const defaultLang = this.translate.getDefaultLang();
    if (defaultLang) return defaultLang;
    const keys = Object.keys(obj);
    return keys[0];
  }
}
