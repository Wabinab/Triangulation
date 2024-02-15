import { Component, inject } from '@angular/core';
import { faSave } from '@fortawesome/free-solid-svg-icons';
import { SharedModule } from '../../shared/shared.module';
import { SharedFormsModule } from '../../shared/shared-forms.module';
import { Http3Service } from '../../services/http3.service';
import { TranslateService } from '@ngx-translate/core';
import { NgbModal } from '@ng-bootstrap/ng-bootstrap';
import { RemindersComponent } from '../cards/reminders/reminders.component';

@Component({
  selector: 'app-template',
  standalone: true,
  imports: [SharedModule, SharedFormsModule],
  templateUrl: './template.component.html',
  styleUrl: './template.component.scss'
})
export class TemplateComponent {
  faSave = faSave;
  stages: any[] = [];
  template: any = {};

  constructor(private http3: Http3Service, public translate: TranslateService) {
    this.get_fivestep();
  }

  save() {

  }

  // ===========================================
  // Sidebar
  curr_stage: number = 0;
  pipeline: any = {};
  sel_stage(value: number) {
    this.curr_stage = value;
    this.pipeline = this.stages.find(c => c.step == value)['pipeline'] ?? [];
  }

  is_active(step: number) {
    return this.curr_stage == step ? 'nav-sidebar-active' : '';
  }

  // =============================================
  // Debug sample five step ray dalio
  async get_fivestep() {
    let value: any = await this.http3.send("/sample_template", "/sample_templ/five_step_ray_dalio.json");
    // let value = await this.http3.send("/", "this confirm will work");
    this.template = JSON.parse(value);
    this.stages = this.template?.stages;
    this.stages.sort(this.compareSteps);
  }

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
}
