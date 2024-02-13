import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
// import { FormsModule, ReactiveFormsModule } from '@angular/forms';
import { HttpClientModule } from '@angular/common/http';
import { TranslateModule } from '@ngx-translate/core';
import { RouterModule } from '@angular/router';
import { FontAwesomeModule } from '@fortawesome/angular-fontawesome';


@NgModule({
  declarations: [],
  imports: [CommonModule, RouterModule],
  exports: [
    CommonModule, 
    // FormsModule,
    // ReactiveFormsModule,
    HttpClientModule,
    TranslateModule,
    RouterModule,
    FontAwesomeModule
  ]
})
export class SharedModule { }
