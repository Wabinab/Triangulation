import { Component, inject, Output, EventEmitter, importProvidersFrom } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { SharedModule } from '../../../shared/shared.module';
import { SharedFormsModule } from '../../../shared/shared-forms.module';
import { NgbModal } from '@ng-bootstrap/ng-bootstrap';
import { CancellationComponent } from '../../cancellation/cancellation.component';
import { TranslateService } from '@ngx-translate/core';
import { Http3Service } from '../../../services/http3.service';
import { Router } from '@angular/router';
import { ToastrService } from 'ngx-toastr';
import { NgSelectModule } from '@ng-select/ng-select';

@Component({
  selector: 'app-new-proj-modal',
  standalone: true,
  imports: [SharedModule, SharedFormsModule, NgSelectModule],
  templateUrl: './new-proj-modal.component.html',
  styleUrl: './new-proj-modal.component.scss'
})
export class NewProjModalComponent {
  myForm: FormGroup;
  templates: any[] = [];
  loading: boolean = false;
  submitting: boolean = false;
  desc_limit = 300;

  @Output() emitCallback = new EventEmitter<any>();
  private modalSvc = inject(NgbModal);

  constructor(private fb: FormBuilder, private translate: TranslateService,
    private http3: Http3Service, private router: Router, private toastr: ToastrService
  ) {
    this.myForm = this.fb.group({
      name: ['', [Validators.required, Validators.maxLength(50)]],
      description: ['', [Validators.maxLength(this.desc_limit)]],
      template_uuid: ['', [Validators.required]]  // more validation later. 
    });
    setTimeout(() => { this.onLoad(); }, 150);  // for webtransport to load. 
  }

  async onLoad() {
    this.loading = true;
    let data = await this.http3.send("/templates", JSON.stringify({}));
    let json_data = JSON.parse(data);
    this.templates = json_data.data;
    if (json_data.err.length > 0) { 
      console.warn("json_data error starts below:");
      console.error(json_data.err); 
      this.translate.get(["newProj.checkF12", "newProj.F12errors"], {}).subscribe((res: any) => {
        this.toastr.error(res["newProj.F12errors"], res["newProj.checkF12"]);
      });
    }
    this.loading = false;
  }
  
  async onSubmit() {
    if (!this.myForm.valid || this.loading || this.submitting) return;
    this.submitting = true;
    const row = {
      name: this.myForm.get('name')!.value,
      description: this.myForm.get('description')!.value,
      template_uuid: this.myForm.get('template_uuid')!.value,
    };

    this.http3.send("/project/new", JSON.stringify(row)).then((data: any) => {
      let filename_json = this.http3.json_handler(data);
      this.router.navigate(["/project"], {queryParams: {
        filename: filename_json.filename
      }});
      this.submitting = false;
    }).catch(err => { this.doErr(err); this.submitting = false; });
  }

  modalCancel: any;
  cancel() {
    if (this.loading || this.submitting) return;
    if (this.is_dirty()) {
      // Use cancellation modal which'll emit to parent component. 
      this.modalCancel = this.modalSvc.open(CancellationComponent);
      this.modalCancel.componentInstance.back_path = "hide modal";
      this.modalCancel.closed.subscribe((res: any) => {
        // console.log(res);
        res["isClosed"] = true;  // from close not dismiss. 
        this.emitCallback.emit(res);
      });
      this.modalCancel.dismissed.subscribe((res: any) => {
        res["isClosed"] = false;
        // this.emitCallback.emit(res);
      });
      return;
    }
    this.emitCallback.emit({});
  }

  is_dirty() {
    let dirty = false;
    Object.keys(this.myForm.controls).forEach(key => {
      const field = this.myForm.get(key)!;
      if (field.dirty && field.touched) { dirty = true; }
    });

    return dirty;
  }

  doErr(err: any) {
    this.loading = false;
    this.submitting = false;
    console.error(err);
    this.toastr.error(err);
    // Waiting for errSvc. 
  }

  charcount: string = '';
  textCounter(event: any) {
    const charcount = this.desc_limit - event.target.value.length;
    const translate_word = charcount >= 0 ? 'newTempl.charRemain' : 'newTempl.charOver';
    this.translate.get(translate_word, {}).subscribe((res: string) => {
      this.charcount = `${Math.abs(charcount)} ${res}`;
    });
  }

  // ==================================================
  // Fetch templates. 

  // Template example: {uuid: 'some_uuid', name: "Meh"}


  
  // ==================================================
  // Debug

  //  get errors() {
  //   const myerrors: any = {};
  //   Object.keys(this.myForm.controls).forEach(key => {
  //     // Get errors of every form control
  //     const form = this.myForm.get(key)!;
  //     if (form.errors != null && (form.dirty || form.touched)) { 
  //       myerrors[key] = form.errors; 
  //     }
  //   });

  //   return Object.keys(myerrors).length ? myerrors : null;
  // }

  // public findInvalidControls() {
  //   const invalid = [];
  //   const controls = this.myForm.controls;
  //   for (const name in controls) {
  //       if (controls[name].invalid) {
  //           invalid.push(controls[name].errors);
  //       }
  //   }
  //   return invalid;
  // }
}
