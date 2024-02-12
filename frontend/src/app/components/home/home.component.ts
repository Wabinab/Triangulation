import { Component } from '@angular/core';
import { SharedModule } from '../../shared/shared.module';
import { ToastrService } from 'ngx-toastr';

@Component({
  selector: 'app-home',
  standalone: true,
  imports: [SharedModule],
  templateUrl: './home.component.html',
  styleUrl: './home.component.scss'
})
export class HomeComponent {

  curr_filter: string = '';
  constructor(private toastr: ToastrService) {}

  new_proj() {

  }

  new_temp() {

  }

  set_filter(value: string) {
    this.curr_filter = value;
    this.toastr.success(value, "Filter chosen");
  }

  is_active(filter_name: string) {
    return this.curr_filter == filter_name ? 'nav-mod-active' : '';
  }
}
