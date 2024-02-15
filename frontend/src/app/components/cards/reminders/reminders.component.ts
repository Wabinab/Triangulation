import { Component, Input } from '@angular/core';
import { Http3Service } from '../../../services/http3.service';
import { SharedModule } from '../../../shared/shared.module';

@Component({
  selector: 'app-reminders',
  standalone: true,
  imports: [SharedModule],
  templateUrl: './reminders.component.html',
  styleUrl: './reminders.component.scss'
})
export class RemindersComponent {
  @Input() id: number = -1;
  @Input() curr_stage: number = 0;
  @Input() filename: string = '';

  items: any;
  loading: boolean = true;

  constructor(private http3: Http3Service) {
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
    let value: any = await this.http3.send("/pipeline", JSON.stringify(data));
    this.items = JSON.parse(value);
  }
}
