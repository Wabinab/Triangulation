import { Component, inject, Input } from '@angular/core';
import { SharedModule } from '../../shared/shared.module';
import { SharedFormsModule } from '../../shared/shared-forms.module';
import { NgbActiveModal } from '@ng-bootstrap/ng-bootstrap';
import { faArrowRight, faCheck } from '@fortawesome/free-solid-svg-icons';
import { TranslateService } from '@ngx-translate/core';

@Component({
  selector: 'app-moveto',
  standalone: true,
  imports: [SharedModule, SharedFormsModule],
  templateUrl: './moveto.component.html',
  styleUrl: './moveto.component.scss'
})
export class MovetoComponent {
  faArrowRight = faArrowRight;
  faTick = faCheck;
  bsModalRef = inject(NgbActiveModal); 

  // From and To are 1 based. 
  @Input() list_names: string[];
  @Input() from: number;
  public to: number;

  constructor(private translate: TranslateService) {
    setTimeout(() => {
      this.translate.get('moveto.empty', {}).subscribe((res: string) => {
        this.list_names = Array.from(this.list_names, item => item || `(${res})`);
      });
    }, 250);
  }

  get_name(index: number) {
    if (index < 1) index = 1;
    else if (index >= this.list_names.length) index = this.list_names.length;
    return this.list_names[index-1];
  }

  okay() {
    if (this.to == undefined) { this.bsModalRef.dismiss(); return; }
    if (this.to < 1) this.to = 1;
    else if (this.to >= this.list_names.length) this.to = this.list_names.length;
    this.bsModalRef.close({ ty: this.to - 1 });  // from 1-based to 0-based. 
  }
}
