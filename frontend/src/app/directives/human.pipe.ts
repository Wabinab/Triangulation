import { Pipe, PipeTransform } from '@angular/core';

/* 
 * Converts camelCase, snake_case, and hyphen-case to human readable format. 
 * Usage: value | human
 * {{ 'CamelCase' | human }} becomes Camel Case.
 * {{ 'camelCase' | human }} becomes Camel Case.
*/
@Pipe({
  name: 'human',
  standalone: true
})
export class HumanPipe implements PipeTransform {

  transform(name: string): unknown {
    var words = name.match(/[A-Za-z][^_\-A-Z]*/g) || [];
    return words.map(this.capitalize).join(" ");
  }

  capitalize(word: string) {
      return word.charAt(0).toUpperCase() + word.substring(1);
  }

}
