import { Component, Input, inject } from '@angular/core';
import { SharedModule } from '../../shared/shared.module';
import { SharedFormsModule } from '../../shared/shared-forms.module';
import { NgbActiveModal } from '@ng-bootstrap/ng-bootstrap';
import { Http3Service } from '../../services/http3.service';
import { FormArray, FormBuilder, FormGroup, Validators } from '@angular/forms';
import { TranslateService } from '@ngx-translate/core';
import { ToastrService } from 'ngx-toastr';
import { UppercaseDirective } from '../../directives/uppercase.directive';
import { faAdd } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeModule } from '@fortawesome/angular-fontawesome';

@Component({
  selector: 'app-kelly',
  standalone: true,
  imports: [SharedModule, SharedFormsModule, FontAwesomeModule, UppercaseDirective],
  templateUrl: './kelly.component.html',
  styleUrl: './kelly.component.scss'
})
export class KellyComponent {
  bsModalRef = inject(NgbActiveModal);

  faAdd = faAdd;

  @Input() id: number = -1;
  @Input() curr_stage: number = 0;
  @Input() filename: string = '';

  is_new = true;
  public myForm: FormGroup;
  max_transaction = 3;

  constructor(private http3: Http3Service, private fb: FormBuilder, 
    private translate: TranslateService, private toastr: ToastrService
  ) {
    this.myForm = fb.group({
      // k_perc: [{value: 0, disabled: true}],
      k_W: [{value: 0, disabled: true}, [Validators.min(0), Validators.max(1)]],
      k_R: [{value: 1, disabled: true}, [Validators.min(this.min_threshold)]],  // denominator cannot be exactly 0. 

      transactions: fb.array([])
    });

    // NO AUTOSAVE!!! 
  }

  // ========================================================
  min_threshold = 0.00001;
  get transactions() {
    const t = this.myForm.get('transactions') as FormArray;
    return t['controls'];
  }

  add_transaction() {
    let t = this.myForm.get('transactions') as FormArray;
    t.insert(0, this.fb.group({  // insert to the beginning. 
      coin: ['', [Validators.required, Validators.minLength(1), Validators.maxLength(10)]],
      buysell: [-1, [Validators.required]], // parsing this requires parseInt() later.
      price: [0, [Validators.required, Validators.min(this.min_threshold)]],
      amt: [0, [Validators.required, Validators.min(this.min_threshold)]],
      
      // These aren't required, if purely "sell"
      // sell_1: [{value: -1, disabled: true}, [Validators.min(this.min_threshold)]],
      price_1: [, [Validators.min(this.min_threshold)]],
      amt_1: [, [Validators.min(this.min_threshold)]],

      // Prediction probability, for brier score. 
      pred_prob: [0.99, [Validators.min(0), Validators.max(1)]]
    }));

    // Remove extra. 
    if (t.length > this.max_transaction) t.removeAt(t.length - 1); 
  }

  // ========================================================
  onSubmit() {

  }

  // ========================================================
  calc_kelly_perc() {
    const W = this.myForm.get('k_W')!.value;
    const R = this.myForm.get('k_R')!.value;
    if (R === 0) {
      this.toastr.error('Win/Loss Ratio cannot be 0.');
      return 0;
    }
    return Math.round(1000 * (W - ((1 - W) / R))) / 1000;
  }

  calc_brier_score() {
    let t = this.myForm.get('transactions')!.value;
    let summed_value = t.map((c: any) => {
      if (c.pred_prob === null || c.price_1 === null || c.amt_1 === null) return 0;
      let o_t = c.price_1 >= c.price ? 1 : 0;
      return Math.round(((c.pred_prob - o_t) ** 2) * 100_000) / 100_000
    }).reduce((partialSum: any, a: any) => partialSum + a, 0);;
    let N = t.filter((c: any) => c.pred_prob !== null && c.price_1 !== null && c.amt_1 !== null).length;
    return Math.round(summed_value / N * 100_000) / 100_000;
  }
}
