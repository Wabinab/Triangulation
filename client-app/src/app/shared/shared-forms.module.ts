import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule, ReactiveFormsModule } from '@angular/forms';
import { ErrorComponent } from '../components/error/error.component';
import { CancellationComponent } from '../components/cancellation/cancellation.component';



@NgModule({
  declarations: [],
  imports: [
    CommonModule, 
    ErrorComponent, 
    CancellationComponent
  ],
  exports: [
    FormsModule,
    ReactiveFormsModule,
    ErrorComponent, 
    CancellationComponent
  ]
})
export class SharedFormsModule { }
