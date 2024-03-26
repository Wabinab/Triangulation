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
  private child_also: boolean;  // this will also apply same class to DIRECT children (only one level below).
  constructor(public elementRef: ElementRef) {
    setTimeout(() => {
      const classes = this.hoverClass.split("|");
      this.class_hover = classes[0].trim();
      this.class_normal = classes[1].trim();
      this.child_also = classes[2] && classes[2].trim() === "true" ? true : false;
      // this.elementRef.nativeElement.classList.add(this.class_normal);
    }, 100);
  }

  @HostListener('mouseenter') onMouseEnter() {
    this.elementRef.nativeElement.classList.remove(this.class_normal);
    this.elementRef.nativeElement.classList.add(this.class_hover);
    if (this.child_also) {
      Array.from(this.elementRef.nativeElement.children).forEach((el: any) => {
        el.classList.remove(this.class_normal);
        el.classList.add(this.class_hover);
      });
    }
  }

  @HostListener('mouseleave') onMouseLeave() {
    this.elementRef.nativeElement.classList.remove(this.class_hover);
    this.elementRef.nativeElement.classList.add(this.class_normal);
    if (this.child_also) {
      Array.from(this.elementRef.nativeElement.children).forEach((el: any) => {
        el.classList.remove(this.class_hover);
        el.classList.add(this.class_normal);
      });
    }
  }
}
