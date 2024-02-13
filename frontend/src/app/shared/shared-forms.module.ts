import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule, ReactiveFormsModule } from '@angular/forms';
import { ErrorComponent } from '../components/error/error.component';



@NgModule({
  declarations: [],
  imports: [
    CommonModule, 
    ErrorComponent
  ],
  exports: [
    FormsModule,
    ReactiveFormsModule,
    ErrorComponent
  ]
})
export class SharedFormsModule { }
