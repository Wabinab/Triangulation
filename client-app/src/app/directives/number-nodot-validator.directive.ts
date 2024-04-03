import { Directive, HostListener } from '@angular/core';
import { NgControl } from '@angular/forms';

@Directive({
  selector: '[appNumberEndWithDot]',
  standalone: true
})
// This only works FORWARD, not BACKWARD. 
// I.e. if your value is 86.4, and you press backspace, you GG. 
// That's deliberate, because backspace not necessarily delete dot, but can be changing value.
export class NumberNodotValidatorDirective {

  just_changed = false;
  orig_val = '';
  keydown_val = '';
  constructor(private control: NgControl) { }

  @HostListener('document:keydown', ['$event']) onKeydownHandler(event: KeyboardEvent) {
    this.keydown_val = event.key;
  }

  @HostListener('input', ['$event.target']) onInput(input: HTMLInputElement): void {
    let value = input.value;
    if ([null, ""].includes(value) && this.keydown_val === '.') {
      this.control.control?.setValue(this.orig_val + '.0');
      input.type = 'text';
      const caretPos = input.value.length - 1;
      input.setSelectionRange(caretPos, caretPos);
      input.type = 'number';
      this.just_changed = true;
      return;
    }
    this.orig_val = value;
    if (this.just_changed) {
      this.control.control?.setValue(parseFloat(input.value.slice(0, -1)))
      this.just_changed = false;
    }
  }
}

// import { AbstractControl, ValidationErrors, ValidatorFn } from "@angular/forms";

// /* Only require this IF NOT USING Validators.required already. */
// export function NumberNoDotValidator(): ValidatorFn {
//   return (control: AbstractControl): ValidationErrors | null => {
//     const value = control.getRawValue();
//     console.log(value);
//     if (!value) return null;
//     // If "67.", it'll match with "7." since single digit.
//     const invalidNumeric = /[0-9].$/.test(value);
//     return invalidNumeric ? { invalidNumeric: true} : null;
//   }
// }