import { Component } from '@angular/core';
import { SharedModule } from '../../shared/shared.module';
import { SharedFormsModule } from '../../shared/shared-forms.module';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { Http3Service } from '../../services/http3.service';
import { ToastrService } from 'ngx-toastr';
import { TranslateService } from '@ngx-translate/core';
import { Routes } from '../../models/routes';

@Component({
  selector: 'app-export',
  standalone: true,
  imports: [SharedModule, SharedFormsModule],
  templateUrl: './export.component.html',
  styleUrl: './export.component.scss'
})
export class ExportComponent {
  public myForm: FormGroup;
  loading: boolean = false;
  uuid = "";
  title = "Untitled";
  
  constructor(private fb: FormBuilder, private http3: Http3Service, 
    private toastr: ToastrService, private translate: TranslateService) {
    this.myForm = fb.group({
      folder: ['', Validators.required],
      filename: ['', [Validators.required, Validators.maxLength(255)]]
    });
  }

  try_get() {
    // let row = { filename: "T_018e6000-1f13-7d6d-beb1-2cf853a52fba.json.zl" }
    let row = { uuid: this.uuid };
    this.loading = true;
    this.http3.send_byte_ret(Routes.TExport, JSON.stringify(row)).then(({ value }) => {
      let decoded_val = new TextDecoder().decode(value);
      if (decoded_val[0] === "{" && decoded_val[decoded_val.length-1] == "}") {
        this.http3.json_handler(decoded_val);
        return;
      }
      // console.log(value);
      const blob = new Blob([value]);
      const url = window.URL.createObjectURL(blob);
      const link = document.createElement("a");
      link.href = url;
      link.download = this.filename_safe(this.title);
      link.click();
      this.loading = false;
    }).catch(err => { this.doErr(err); this.loading = false; });
  }

  onSubmit() {

  }

  // ========================================
  doErr(err: any) {
    console.error(err);
    if (typeof(err) === 'string') this.toastr.error(this.translate.instant(err || ''));
    else this.toastr.error(err);
  }

  private filename_safe(normal_name: string) {
    let safe_filename = normal_name.replace(/[^a-z0-9]/gi, '_').toLowerCase();
    return safe_filename + ".json.zl";
  }
}
