import { Component } from '@angular/core';
import { faSave } from '@fortawesome/free-solid-svg-icons';
import { SharedModule } from '../../shared/shared.module';
import { SharedFormsModule } from '../../shared/shared-forms.module';
import { Http3Service } from '../../services/http3.service';

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

  constructor(private http3: Http3Service) {
    this.get_fivestep();
  }

  save() {

  }

  // ===========================================
  // Sidebar
  curr_stage: number = 0;
  sel_stage(value: number) {
    this.curr_stage = value;
  }

  is_active(step: number) {
    return this.curr_stage == step ? 'nav-sidebar-active' : '';
  }

  // =============================================
  // Debug sample five step ray dalio
  async get_fivestep() {
    let value: any = await this.http3.send("/sample_template", "/sample_templ/five_step_ray_dalio.json");
    // let value = await this.http3.send("/", "this confirm will work");
    // console.log(value);
    this.stages = JSON.parse(value).stages.map((c: any) => {
      return { step: c.step, name: c.name }
    });
    this.stages.sort(this.compareSteps);
  }

  private compareSteps(a: any, b: any) {
    return a.step - b.step;
  }
}
