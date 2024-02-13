import { CommonModule } from '@angular/common';
import { Component, OnInit, Input } from '@angular/core';
import { ThemeManager } from '../../services/theme-manager.service';
import { TranslateModule, TranslateService } from '@ngx-translate/core';

@Component({
  selector: 'app-error',
  standalone: true,
  imports: [CommonModule, TranslateModule],
  templateUrl: './error.component.html',
  styleUrl: './error.component.scss'
})
export class ErrorComponent implements OnInit {

  constructor(private theme: ThemeManager, private translate: TranslateService) { return; }

  ngOnInit(): void {
    this._form_control = this.form.controls[this.control];
  }

  @Input() form: any;
  @Input() control: any;
  @Input() custom_message: any;  // WILL OVERWRITE ALL ERR MSG from being printed. 
  @Input() error_type: any;  // USE WITH custom_message ONLY! MUST EXIST otherwise fail w/o rescue. 
  _form_control!: any;

  get display_errors() {
    
    if (this._form_control.dirty || this._form_control.touched) {
      return this.render_message(this._form_control.errors ?? {});
      // return this.custom_message;
    }
    return [];
  }

  render_message(err_msg: any) {
    // Check required
    let aggregate_msgs: string[] = [];

    // if (err_msg.required) {
    //   aggregate_msgs.push("err.required");
    // }

    if (err_msg.min) {
      // aggregate_msgs.push(`Minimum value: ${err_msg.min.min}`);
      this.translate.get('err.min', {value: err_msg.min.min}).subscribe(res => {
        aggregate_msgs.push(res);
      })
    }

    if (err_msg.max) {
      // aggregate_msgs.push(`Maximum value: ${err_msg.max.max}`);
      this.translate.get('err.max', {value: err_msg.max.max}).subscribe(res => {
        aggregate_msgs.push(res);
      })
    }

    if (err_msg.minlength) {
      this.translate.get('err.minLength', {value: err_msg.minlength.requiredLength}).subscribe(res => {
        aggregate_msgs.push(res);
      })
    }

    if (err_msg.maxlength) {
      this.translate.get('err.maxLength', {value: err_msg.maxlength.requiredLength}).subscribe(res => { 
        aggregate_msgs.push(res);
      });
    }

    // if (err_msg.email) {
    //   aggregate_msgs.push(`Invalid email.`);
    // }

    // if (err_msg.validatePhoneNumber) {
    //   aggregate_msgs.push("Invalid.");
    // }

    // WILL OVERWRITE ALL ERR MSG rather than push to back. Use with care! 
    if (this.custom_message != undefined) { 
      if (err_msg[this.error_type]) {
        // aggregate_msgs = [this.custom_message];
        this.translate.get(this.custom_message, {}).subscribe(res => {
          aggregate_msgs = [res];
        });
      }
    }

    return aggregate_msgs
  }

  get_class() {
    return this.theme.get_curr_mode() == 'light' 
    ? 'bg-danger bg-opacity-25'
    : 'bg-danger bg-opacity-75';
  }

}
