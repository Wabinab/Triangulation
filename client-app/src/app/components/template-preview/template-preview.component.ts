import { Component, Input, inject } from '@angular/core';
import { SharedModule } from '../../shared/shared.module';
import { FontAwesomeModule } from '@fortawesome/angular-fontawesome';
import { SharedFormsModule } from '../../shared/shared-forms.module';
import { Http3Service } from '../../services/http3.service';
import { TranslateService } from '@ngx-translate/core';
import { Routes } from '../../models/routes';
import { NgbActiveModal, NgbModal } from '@ng-bootstrap/ng-bootstrap';
import { CardTypes } from '../../models/card-types';
import { ToastrService } from 'ngx-toastr';
import { HelperService } from '../../services/helper.service';
import { HoverClassDirective } from '../../directives/hover-class.directive';
import { RemindersReadonlyComponent } from '../cards/reminders-readonly/reminders-readonly.component';
import { KellyReadonlyComponent } from '../cards/kelly-readonly/kelly-readonly.component';
import { ChecklistReadonlyComponent } from '../cards/checklist-readonly/checklist-readonly.component';

@Component({
  selector: 'app-template-preview',
  standalone: true,
  imports: [SharedModule, FontAwesomeModule, SharedFormsModule, HoverClassDirective],
  templateUrl: './template-preview.component.html',
  styleUrl: './template-preview.component.scss'
})
export class TemplatePreviewComponent {
  stages: any[] = [];
  @Input() filename: string = '';
  template: any = {};
  loading: boolean = true;

  constructor(private http3: Http3Service, public translate: TranslateService,
    private toastr: ToastrService, public helperSvc: HelperService
  ) {
    this.loading = true;
    setTimeout(() => this.load(), 100);
  }

  async load(curr_stage: number = 0) {
    if (!this.filename) { this.doErr("proj.UndFilename"); this.loading = false; return; }
    this.loading = true;
    const row = { filename: this.filename };
    this.http3.send(Routes.SampleTNlist, JSON.stringify(row)).then((value: any) => {
      this.template = this.http3.json_handler(value);
      this.stages = this.template?.stages;
      this.stages.sort(this.compareSteps);
      this.pipeline = this.stages[curr_stage] ? this.stages[curr_stage]['pipeline'] : [];
      this.loading = false;
    }).catch(err => { this.doErr(err); this.loading = false; })
  }

  private compareSteps(a: any, b: any) { return a.step - b.step; }

  // =====================================
  // Sidebar
  curr_stage: number = 0;
  pipeline: any = [];

  sel_stage(value: number) {
    this.curr_stage = value;
    if (!this.stages[value]) { this.pipeline = []; return; }
    this.pipeline = this.stages[value]["pipeline"] ?? [];
  }

  is_active_stage(step: number) { return this.curr_stage == step ? 'nav-sidebar-active' : ''; }

  // =========================================
  // Modals
  bsModalRef = inject(NgbActiveModal);
  private modalSvc = inject(NgbModal);

  close() { this.bsModalRef.dismiss(); }

  openTemplate(id: number, ty: number) {
    if (ty == CardTypes.Reminders) { this.openReminder(id); return; }
    if (ty == CardTypes.Kelly) { this.openKelly(id); return; }
    if (ty == CardTypes.Checklist) { this.openChecklist(id); return; }
  }

  modalCard: any;
  openModal(id: number, component: any, options: any) {
    this.modalCard = this.modalSvc.open(component, options);
    this.modalCard.componentInstance.id = id;
    this.modalCard.componentInstance.curr_stage = this.curr_stage;
    this.modalCard.componentInstance.filename = this.filename;
    this.modalCard.componentInstance.read_only = true;
  }

  openReminder(id: number) {
    this.openModal(id, RemindersReadonlyComponent, { fullscreen: 'sm', size: 'xl' });
  }
  openKelly(id: number) {
    this.openModal(id, KellyReadonlyComponent, { fullscreen: 'lg', size: 'xl'});
  }
  openChecklist(id: number) {
    this.openModal(id, ChecklistReadonlyComponent, { fullscreen: 'lg', size: 'xl' });
  }

  // =========================================
  doErr(err: any) {
    console.error(err);
    if (typeof(err) === 'string') this.toastr.error(this.translate.instant(err || ''));
    else this.toastr.error(err);
  }
}
