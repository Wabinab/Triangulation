import { Directive, ElementRef, HostListener} from '@angular/core';
import { NgControl } from '@angular/forms';

@Directive({
  selector: '[appUppercase]',
  standalone: true
})
export class UppercaseDirective {

  constructor(private control: NgControl) { }

  @HostListener('input', ['$event.target']) onInput(input: HTMLInputElement): void {
    const caretPos = input.selectionStart;
    this.control.control?.setValue(input.value.toUpperCase());
    input.setSelectionRange(caretPos, caretPos);
  }
}
