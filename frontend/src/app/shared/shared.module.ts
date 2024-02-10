import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule, ReactiveFormsModule } from '@angular/forms';
import { HttpClientModule } from '@angular/common/http';
import { TranslateModule } from '@ngx-translate/core';
import { BrowserModule } from '@angular/platform-browser';


@NgModule({
  declarations: [],
  imports: [CommonModule],
  exports: [
    CommonModule, 
    FormsModule,
    ReactiveFormsModule,
    HttpClientModule,
    TranslateModule,
    BrowserModule
  ]
})
export class SharedModule { }
