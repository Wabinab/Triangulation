import { Component, inject, signal } from '@angular/core';
import { SharedModule } from '../../shared/shared.module';
import { ToastrService } from 'ngx-toastr';
import { faArrowDown, faCheck, faFileExport, faRoad, faRotate, faRoute, faTrashAlt } from '@fortawesome/free-solid-svg-icons';
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
import { NgbModal } from '@ng-bootstrap/ng-bootstrap';
import { CancellationComponent } from '../cancellation/cancellation.component';
import { GithubService } from '../../services/github.service';
import { faClone } from '@fortawesome/free-regular-svg-icons';
import { TemplatePreviewComponent } from '../template-preview/template-preview.component';
// import { ExportComponent } from '../export/export.component';

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
  faSync = faRotate;
  faDownload = faArrowDown;
  faClone = faClone;
  faDelete = faTrashAlt;
  faTick = faCheck;
  faExport = faFileExport;

  deferProjClicked = signal(false);
  deferTemplClicked = signal(false);
  private modalSvc = inject(NgbModal);

  curr_view: HomeView = HomeView.Home;  // home, new (proj/temp) views.
  // curr_filter: string = 'proj';
  curr_filter: HomeFilter = HomeFilter.Project;
  HomeFilter = HomeFilter;
  items: any[] = [];
  // page = { page_no: 0, page_size: 1, total_count: 0 }
  page = { currentPage: 1, itemsPerPage: 12, totalItems: 0 };
  loading = false;

  constructor(private toastr: ToastrService, private http3: Http3Service, 
    private router: Router, private translate: TranslateService,
    private github: GithubService
  ) {
    // Default is project, so we get project and fill items. 
    setTimeout(() => {
      this.get_projects(true);  // give time for service to load. 
      // this.get_templates(true);
      // this.get_sample_templates(true);
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
    this.get_filter();
    // this.toastr.success(value, "Filter chosen");
  }

  change_page(page_no: any) {
    this.page.currentPage = page_no;
    this.get_filter();
  }

  // Can only be called if isn't in sample template. 
  redirect_to(uuid: string) {
    if (![HomeFilter.Template, HomeFilter.Project].includes(this.curr_filter)) {
      this.doErr("home.WrongFilter");
    }
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
  get_filter() {
    if (this.curr_filter == HomeFilter.Template) this.get_templates();
    if (this.curr_filter == HomeFilter.Project) this.get_projects();
    if (this.curr_filter == HomeFilter.SampleTemplate) this.get_sample_templates();
  }

  get_templates(retry: boolean = false) {
    this.get_internal(Routes.Ts, retry);
  }

  get_projects(retry: boolean = false) {
    this.get_internal(Routes.Ps, retry);
  }

  get_sample_templates(retry: boolean = false) {
    // this.get_internal('', retry);
    if (this.loading) { this.wait(); return; }
    this.loading = true;
    this.github.get_index().subscribe((res: any) => {
      this.items = res;
      this.loading = false;
      this.sync_index();
    }, (err: any) => { this.doErr(err); this.loading = false; });
  }

  timeout_count = 0;
  timeout_threshold = 300;
  private get_internal(route: string, retry: boolean = false) {
    this.timeout_count++;
    if (this.timeout_count > this.timeout_threshold) {
      this.doErr("error.Timeout");
      this.timeout_count = 0;
      return;
    }
    const row = {
      page_no: this.page.currentPage - 1,
      page_size: this.page.itemsPerPage
    };
    this.loading = true;
    this.http3.send(route, JSON.stringify(row)).then(async (_res: any) => {
      let res = this.http3.json_handler(_res);
      this.items = res.data.reverse();
      if (res.err && res.err.length > 0) {
        this.translate.get(["newProj.checkF12", "newProj.F12errors"], {}).subscribe((res: any) => {
          this.toastr.error(res["newProj.F12errors"], res["newProj.checkF12"]);
        });
      }
      this.page.totalItems = res.total_count;
      this.timeout_count = 0;
      this.loading = false;
    }).catch((err: any) => {
      if (retry) { setTimeout(() => this.get_internal(route, retry), 1000); return; }
      this.doErr(err);
      this.loading = false;
    });
  }

  // ======================================================
  // The "clone" from sample template to template use another function. 
  clone_item(uuid: string) {
    if (![HomeFilter.Template, HomeFilter.Project].includes(this.curr_filter)) {
      this.doErr("home.WrongFilter");
    }
    if (this.loading) { this.wait(); return; }
    this.loading = true;
    const route = this.curr_filter === HomeFilter.Template ? Routes.TClone : Routes.PClone;
    const row = { uuid: uuid };
    this.http3.send(route, JSON.stringify(row)).then((_res: any) => {
      let res = this.http3.json_handler(_res);
      // this.get_projects();  // no need refresh, since we're gonna redirect page. 
      this.loading = false;

      const type_name = this.curr_filter == HomeFilter.Template ? 'template': 'project'
      this.router.navigate([`/${type_name}`], {queryParams: {
        filename: res.filename
      }});
    }).catch((err: any) => { this.doErr(err); this.loading = false; })
  }

  modalDelete: any;
  delete_item(uuid: string) {
    if (![HomeFilter.Template, HomeFilter.Project].includes(this.curr_filter)) {
      this.doErr("home.WrongFilter"); return;
    }
    if (this.loading) { this.wait(); return; }
    this.modalDelete = this.modalSvc.open(CancellationComponent, { fullscreen: 'sm' });
    this.modalDelete.componentInstance.back_path = "hide modal";  // no need redirect.
    this.modalDelete.componentInstance.back_dismiss = true;
    this.modalDelete.closed.subscribe((res: any) => {
      this.loading = true;
      const route = this.curr_filter === HomeFilter.Template ? Routes.TDel : Routes.PDel;
      const row = { uuid: uuid };
      this.http3.send(route, JSON.stringify(row)).then((_res: any) => {
        let res = this.http3.json_handler(_res);
        this.get_filter();  // we refresh since we're not redirecting.
        this.loading = false;
      }).catch((err: any) => { this.doErr(err); this.loading = false; })
    });
  }

  // modalExport: any;
  export_item(uuid: string, title: string) {
    if (this.curr_filter != HomeFilter.Template) { this.doErr("home.OnlyTemplate"); return; }
    if (this.loading) { this.wait(); return; }
    // this.modalExport = this.modalSvc.open(ExportComponent, { fullscreen: 'sm' });
    // this.modalExport.componentInstance.uuid = uuid;
    // this.modalExport.componentInstance.title = title;
    // this.modalExport.closed.subscribe((res: any) => {
    //   this.get_filter();
    // });
    let row = { uuid: uuid };
    this.loading = true;
    this.http3.send_byte_ret(Routes.TExport, JSON.stringify(row)).then(({ value }) => {
      let decoded_val = new TextDecoder().decode(value);
      if (decoded_val[0] === "{" && decoded_val[decoded_val.length-1] == "}") {
        this.http3.json_handler(decoded_val);
        this.loading = false;
        return;
      }
      // console.log(value);
      const blob = new Blob([value]);
      const url = window.URL.createObjectURL(blob);
      const link = document.createElement("a");
      link.href = url;
      link.download = this.filename_safe(title);
      link.click();
      this.loading = false;
    }).catch(err => { this.doErr(err); this.loading = false; });
  }

  private filename_safe(normal_name: string) {
    let safe_filename = normal_name.replace(/[^a-z0-9]/gi, '_').toLowerCase();
    return safe_filename + ".json.zl";
  }

  // ==========================================================
  // Sample template
  // Sync after download from folder. 
  downloaded_sample: string[] = [];
  sync_index() {
    this.loading = true;
    this.http3.send(Routes.SampleList, "{}").then((res: any) => {
      this.downloaded_sample = this.http3.json_handler(res).data;
      this.loading = false;
    }).catch((err: any) => {this.doErr(err); this.loading = false });
  }

  // Download the file, then open up template.component as modal to preview sample. 
  // Auto-download file when clicked. 
  modalSample: any;
  preview_sample(filename: string) {
    if (!this.downloaded(filename)) { this.doErr("home.DownloadFirst"); return; }
    // Either use TemplateComponent, or create TemplateReadOnlyComponent for specifics. 
    this.modalSample = this.modalSvc.open(TemplatePreviewComponent, {
      // backdrop: 'static',
      fullscreen: 'lg',
      size: 'xl'
    });
    this.modalSample.componentInstance.filename = filename;
    // this.modalSample.componentInstance.read_only = true;  
  }

  clone_sample(filename: string) {
    this.loading = true;
    let row = { filename: filename };

    this.http3.send(Routes.SampleClone, JSON.stringify(row)).then((res: any) => {
      let retval = this.http3.json_handler(res);
      let filename = retval.filename;
      this.router.navigate([`/template`], {queryParams: {
        filename: filename
      }});
      this.loading = false;
    }).catch(err => { this.doErr(err); this.loading = false; });
  }

  download_sample(filename: string) {
    // this.github.download_object(filename);
    this.loading = true;
    let row = { filename: filename };

    this.http3.send(Routes.SampleDownload, JSON.stringify(row)).then((res: any) => {
      let retval: any = this.http3.json_handler(res);
      console.warn(retval);
      this.toastr.info(retval.msg);
      this.loading = false;
      this.get_sample_templates();
    }).catch(err => { this.doErr(err); this.loading = false; });
  }

  downloaded(filename: string) {
    return this.downloaded_sample.includes(filename);
  }

  // ==========================================================
  doErr(err: any) {
    this.loading = false;
    console.error(err);
    if (typeof(err) === 'string') this.toastr.error(this.translate.instant(err || ''));
    else this.toastr.error(err);
  }

  wait() {
    this.toastr.info(this.translate.instant("wait"));
  }

  get_class() {
    return this.is_active(HomeFilter.Project) ? "row row-cols-2 g-0" : "row row-cols-3 g-0"
  }

  // ============================================================
  // Debug
  // test_get_index() {
  //   this.github.get_index();
  // }
}
