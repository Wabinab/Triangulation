import { Component, OnInit } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { SharedModule } from '../../../shared/shared.module';
import { SharedFormsModule } from '../../../shared/shared-forms.module';

@Component({
  selector: 'app-new-proj-modal',
  standalone: true,
  imports: [SharedModule, SharedFormsModule],
  templateUrl: './new-proj-modal.component.html',
  styleUrl: './new-proj-modal.component.scss'
})
export class NewProjModalComponent implements OnInit {
  myForm: FormGroup;
  templates: any[] = [{cid: 'some_cid', name: "Meh"}];
  loading: boolean = false;
  submitting: boolean = false;

  constructor(private fb: FormBuilder) {
    this.myForm = this.fb.group({
      name: ['', [Validators.required, Validators.maxLength(50)]],
      description: ['', [Validators.maxLength(255)]],
      template_cid: ['', [Validators.required]]  // more validation later. 
    });
  }

  ngOnInit(): void {}

  onSubmit() {

  }

  cancel() {
    // Use cancellation modal which'll emit to parent component. 
  }

  // ==================================================
  // Fetch templates. 


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
