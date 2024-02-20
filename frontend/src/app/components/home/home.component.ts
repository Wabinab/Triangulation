import { Component, signal } from '@angular/core';
import { SharedModule } from '../../shared/shared.module';
import { ToastrService } from 'ngx-toastr';
import { faRoad, faRoute } from '@fortawesome/free-solid-svg-icons';
import { HomeView } from '../../models/home-view';
import { NewProjModalComponent } from './new-proj-modal/new-proj-modal.component';
import { NewTemplModalComponent } from './new-templ-modal/new-templ-modal.component';
import { FontAwesomeModule } from '@fortawesome/angular-fontawesome';

@Component({
  selector: 'app-home',
  standalone: true,
  imports: [SharedModule, FontAwesomeModule, NewProjModalComponent, NewTemplModalComponent],
  templateUrl: './home.component.html',
  styleUrl: './home.component.scss'
})
export class HomeComponent {
  faProj = faRoad;
  faTempl = faRoute;
  deferProjClicked = signal(false);
  deferTemplClicked = signal(false);

  curr_view: HomeView = HomeView.Home;  // home, new (proj/temp) views.
  curr_filter: string = 'proj';
  constructor(private toastr: ToastrService) {}

  new_proj() {
    // Cancellation before continue. 
    if (this.curr_view != HomeView.Home) { }
    this.deferProjClicked.set(true);
    this.curr_view = HomeView.NewProj;
  }

  new_templ() {
    if (this.curr_view != HomeView.Home) { }
    this.deferTemplClicked.set(true);
    this.curr_view = HomeView.NewTempl;
  }

  // ===============================================================
  // Filter
  set_filter(value: string) {
    this.curr_filter = value;
    this.toastr.success(value, "Filter chosen");
  }

  is_active(filter_name: string) {
    return this.curr_filter == filter_name ? 'nav-filter-active' : '';
  }

  // ================================================================
  // View

  tab_active(id: number) {
    return this.curr_view == id ? 'nav-tab-active' : '';
  }

  projCallback(event: any) {
    // console.log(event);
    this.curr_view = HomeView.Home;
  }

  templCallback(event: any) {
    this.curr_view = HomeView.Home;
  }
}
