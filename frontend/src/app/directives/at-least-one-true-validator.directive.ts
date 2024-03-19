// import { Directive } from '@angular/core';
import { AbstractControl, ValidationErrors, ValidatorFn } from '@angular/forms';

export function atLeastOneTrueValidator(): ValidatorFn {
  return (control: AbstractControl): ValidationErrors | null => {
    const value = control.value;
    if (!value) return null;
    return value.includes(true) ? null : { atLeastOneTrue: true }
  }
}

// @Directive({
//   selector: '[appAtLeastOneTrueValidator]',
//   standalone: true
// })
// export class AtLeastOneTrueValidatorDirective {

//   constructor() { }

// }

