import { Component, signal } from '@angular/core';
import { SharedModule } from '../../shared/shared.module';
import { ToastrService } from 'ngx-toastr';
import { faRoad, faRoute } from '@fortawesome/free-solid-svg-icons';
import { HomeView } from '../../models/home-view';
import { NewProjModalComponent } from './new-proj-modal/new-proj-modal.component';
import { NewTemplModalComponent } from './new-templ-modal/new-templ-modal.component';
import { FontAwesomeModule } from '@fortawesome/angular-fontawesome';
import { Http3Service } from '../../services/http3.service';
import { HomeFilter } from '../../models/home-filter';
import { Router } from '@angular/router';
import { NgxPaginationModule } from 'ngx-pagination';
import { HoverClassDirective } from '../../directives/hover-class.directive';
import { TranslateService } from '@ngx-translate/core';
import { Routes } from '../../models/routes';

@Component({
  selector: 'app-home',
  standalone: true,
  imports: [SharedModule, FontAwesomeModule, NgxPaginationModule, NewProjModalComponent, 
    NewTemplModalComponent, HoverClassDirective],
  templateUrl: './home.component.html',
  styleUrl: './home.component.scss'
})
export class HomeComponent {
  faProj = faRoad;
  faTempl = faRoute;
  deferProjClicked = signal(false);
  deferTemplClicked = signal(false);

  curr_view: HomeView = HomeView.Home;  // home, new (proj/temp) views.
  // curr_filter: string = 'proj';
  curr_filter: HomeFilter = HomeFilter.Project;
  HomeFilter = HomeFilter;
  items: any[] = [];
  // page = { page_no: 0, page_size: 1, total_count: 0 }
  page = { currentPage: 1, itemsPerPage: 12, totalItems: 0 };
  loading = false;

  constructor(private toastr: ToastrService, private http3: Http3Service, 
    private router: Router, private translate: TranslateService
  ) {
    // Default is project, so we get project and fill items. 
    setTimeout(() => {
      this.get_projects(true);  // give time for service to load. 
    }, 100);
  }

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
  set_filter(value: HomeFilter) {
    this.curr_filter = value;
    this.get_template_or_project();
    // this.toastr.success(value, "Filter chosen");
  }

  change_page(page_no: any) {
    this.page.currentPage = page_no;
    this.get_template_or_project();
  }

  redirect_to(uuid: string) {
    this.loading = true;
    const type_name = this.curr_filter == HomeFilter.Template ? 'template': 'project'
    const row = {
      type_name: type_name,
      uuid: uuid
    };
    this.http3.send(Routes.GenFilename, JSON.stringify(row)).then((_res: any) => {
      let res = this.http3.json_handler(_res);
      this.router.navigate([`/${type_name}`], {queryParams: {
        filename: res.filename
      }});
      this.loading = false;
    }).catch((err: any) => { this.doErr(err); this.loading = false; })
  }

  is_active(filter_name: HomeFilter) {
    return this.curr_filter == filter_name ? 'nav-sidebar-active' : '';
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

  // =================================================================
  // Get data
  get_template_or_project() {
    if (this.curr_filter == HomeFilter.Template) this.get_templates();
    if (this.curr_filter == HomeFilter.Project) this.get_projects();
  }

  get_templates(retry: boolean = false) {
    this.get_internal(Routes.Ts);
  }

  get_projects(retry: boolean = false) {
    this.get_internal(Routes.Ps, retry);
  }

  private get_internal(route: string, retry: boolean = false) {
    const row = {
      page_no: this.page.currentPage - 1,
      page_size: this.page.itemsPerPage
    };
    // console.log(row);
    this.http3.send(route, JSON.stringify(row)).then(async (_res: any) => {
      let res = this.http3.json_handler(_res);
      this.items = res.data;
      if (res.err && res.err.length > 0) {
        // this.toastr.error("Check F12 logs", "Error Found");
        this.translate.get(["newProj.checkF12", "newProj.F12errors"], {}).subscribe((res: any) => {
          this.toastr.error(res["newProj.F12errors"], res["newProj.checkF12"]);
        });
      }
      this.page.totalItems = res.total_count;
    }).catch((err: any) => {
      if (retry) { setTimeout(() => this.get_internal(route, retry), 100); return; }
      this.doErr(err);
    });
  }

  doErr(err: any) {
    this.loading = false;
    console.error(err);
    this.toastr.error(err);
  }
}
