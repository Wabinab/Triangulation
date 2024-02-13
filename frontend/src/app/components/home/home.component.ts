import { Component } from '@angular/core';
import { SharedModule } from '../../shared/shared.module';
import { ToastrService } from 'ngx-toastr';
import { faRoad, faRoute } from '@fortawesome/free-solid-svg-icons';
import { HomeView } from '../../models/home-view';
import { NewProjModalComponent } from './new-proj-modal/new-proj-modal.component';

@Component({
  selector: 'app-home',
  standalone: true,
  imports: [SharedModule, NewProjModalComponent],
  templateUrl: './home.component.html',
  styleUrl: './home.component.scss'
})
export class HomeComponent {
  faProj = faRoad;
  faTempl = faRoute;


  curr_view: HomeView = HomeView.Home;  // home, new (proj/temp) views.
  curr_filter: string = 'proj';
  constructor(private toastr: ToastrService) {}

  new_proj() {
    // Cancellation before continue. 
    if (this.curr_view != HomeView.Home) { }
    this.curr_view = HomeView.NewProj;
  }

  new_templ() {
    if (this.curr_view != HomeView.Home) { }
    this.curr_view = HomeView.NewTempl;
  }

  // ===============================================================
  // Filter
  set_filter(value: string) {
    this.curr_filter = value;
    this.toastr.success(value, "Filter chosen");
  }

  is_active(filter_name: string) {
    return this.curr_filter == filter_name ? 'nav-mod-active' : '';
  }

  // ================================================================
  // View
  cancel() {
    // Cancellation modal. 
    // Changing from project to template is considered as cancel. 
    this.curr_view = HomeView.Home;
  }
}
