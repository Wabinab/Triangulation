// https://stackoverflow.com/questions/42633117/how-can-i-add-a-class-to-an-element-on-hover
import { Directive, ElementRef, HostListener, Input } from '@angular/core';

@Directive({
  selector: '[hover-class]',
  standalone: true
})
export class HoverClassDirective {

  @Input('hover-class') hoverClass: string;  // "class-hover | class-non-hover"
  private class_hover: string;
  private class_normal: string;
  constructor(public elementRef: ElementRef) {
    setTimeout(() => {
      const classes = this.hoverClass.split("|");
      this.class_hover = classes[0].trim();
      this.class_normal = classes[1].trim();
      // this.elementRef.nativeElement.classList.add(this.class_normal);
    }, 100);
  }

  @HostListener('mouseenter') onMouseEnter() {
    this.elementRef.nativeElement.classList.remove(this.class_normal);
    this.elementRef.nativeElement.classList.add(this.class_hover);
  }

  @HostListener('mouseleave') onMouseLeave() {
    this.elementRef.nativeElement.classList.remove(this.class_hover);
    this.elementRef.nativeElement.classList.add(this.class_normal);
  }
}
