import { Component, Input, inject } from '@angular/core';
import { SharedModule } from '../../../shared/shared.module';
import { SharedFormsModule } from '../../../shared/shared-forms.module';
import { NgbActiveModal, NgbModal } from '@ng-bootstrap/ng-bootstrap';
import { Http3Service } from '../../../services/http3.service';
import { FormArray, FormBuilder, FormGroup, Validators } from '@angular/forms';
import { TranslateService } from '@ngx-translate/core';
import { ToastrService } from 'ngx-toastr';
import { UppercaseDirective } from '../../../directives/uppercase.directive';
import { faAdd, faXmark } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeModule } from '@fortawesome/angular-fontawesome';
import { faSave } from '@fortawesome/free-regular-svg-icons';
import { CancellationComponent } from '../../cancellation/cancellation.component';
import { NumberNodotValidatorDirective } from '../../../directives/number-nodot-validator.directive';
import { Routes } from '../../../models/routes';
// import { NumberNoDotValidator } from '../../directives/number-nodot-validator.directive';

@Component({
  selector: 'app-kelly',
  standalone: true,
  imports: [SharedModule, SharedFormsModule, FontAwesomeModule, UppercaseDirective, 
    NumberNodotValidatorDirective],
  templateUrl: './kelly.component.html',
  styleUrl: './kelly.component.scss'
})
export class KellyComponent {
  bsModalRef = inject(NgbActiveModal);

  faAdd = faAdd;
  faCross = faXmark;
  faSave = faSave;

  @Input() id: number = -1;
  @Input() curr_stage: number = 0;
  @Input() filename: string = '';

  items: any;
  loading = true;
  submitting = false;
  is_new = true;
  public myForm: FormGroup;
  // max_transaction = 5;

  constructor(private http3: Http3Service, private fb: FormBuilder, 
    private translate: TranslateService, private toastr: ToastrService
  ) {
    this.myForm = fb.group({
      // k_perc: [{value: 0, disabled: true}],
      // k_W: [{value: 0, disabled: true}, [Validators.min(0), Validators.max(1)]],
      // k_R: [{value: 1, disabled: true}, [Validators.min(this.min_threshold)]],  // denominator cannot be exactly 0. 
      title: [, [Validators.required, Validators.minLength(1), Validators.maxLength(50)]],
      // transactions: fb.array([])
    });

    // NO AUTOSAVE!!! 
    setTimeout(() => {
      this.loadData();
    }, 100);
  }

  async loadData() {
    let data = {
      filename: this.filename,
      stage_index: this.curr_stage,
      pipeline_index: this.id
    }

    let value: any = await this.http3.send(Routes.Pi, JSON.stringify(data));
    this.items = JSON.parse(value ?? '{}');
    if (this.items.err && this.items.err == "backend.OOBPipeline") {
      this.is_new = true;
      this.loading = false;
      return;
    }
    this.myForm.get('title')?.setValue(this.items.title);
    this.loading = false;
  }

  // ========================================================
  // min_threshold = 0.00001;
  // min_amt = 0.00000001;  // smaller, because BTC is large value. 
  // get transactions() {
  //   const t = this.myForm.get('transactions') as FormArray;
  //   return t['controls'];
  // }

  // add_transaction() {
  //   console.log("called add transaction")
  //   let t = this.myForm.get('transactions') as FormArray;
  //   t.insert(0, this.fb.group({  // insert to the beginning. 
  //     coin: ['', [Validators.required, Validators.minLength(1), Validators.maxLength(10)]],
  //     buy: [true, [Validators.required]], // parsing this requires parseInt() later.
  //     price: [0, [Validators.required, Validators.min(this.min_threshold)]],
  //     amt: [0, [Validators.required, Validators.min(this.min_amt)]],
      
  //     // These aren't required, if purely "sell"
  //     // sell_1: [{value: -1, disabled: true}, [Validators.min(this.min_threshold)]],
  //     price_1: [, [Validators.min(this.min_threshold)]],
  //     amt_1: [, [Validators.min(this.min_amt)]],

  //     // Prediction probability, for brier score. 
  //     pred_prob: [1, [Validators.min(0), Validators.max(1)]]
  //   }));

  //   // Remove extra. 
  //   if (t.length > this.max_transaction) t.removeAt(t.length - 1); 
  // }

  // upd_amt_1_validator(rowNo: number) {
  //   let min_val = [Validators.min(this.min_amt)];
  //   if (rowNo >= this.transactions.length) { this.doErr("upd_amt_1_validators rowNo OOB."); return; }
  //   let t = this.transactions[rowNo];
  //   let amt = t.get('amt')?.value;
  //   if (amt === null || amt <= this.min_amt) { t.get('amt_1')?.setValidators(min_val); return; }

  //   min_val.push(Validators.max(amt));
  //   t.get('amt_1')?.setValidators(min_val);
  //   t.updateValueAndValidity();
  //   return;
  // }

  // remove_transaction(i: number) {
  //   let t = this.myForm.get('transactions') as FormArray;
  //   t.removeAt(i);
  // }

  // ========================================================
  onSubmit() {
    if (this.submitting || this.loading || this.myForm.invalid) {
      if (this.myForm.invalid) {this.translate.get("err.InvalidForm", {})
      .subscribe((res: any) => { this.doErr(res); }); }
      return;
    }
    this.submitting = true;
    const row = {
      filename: this.filename,
      stage_index: this.curr_stage,
      pipeline_index: this.id,
      title: this.myForm.get('title')!.value
      // transactions: this.myForm.get('transactions')!.value.map((c: any) => {
      //   c.buy = c.buy === "true";
      //   return c;
      // })
    }
    
    this.http3.send(this.is_new ? Routes.PiNew1 : Routes.PiEdit1, JSON.stringify(row))
    .then((res: any) => {
      this.submitting = false;
      this.bsModalRef.close({ ty: this.http3.json_handler(res) });
    }).catch((err: any) => { this.doErr(err); this.submitting = false; });
  }

  private modalSvc = inject(NgbModal);
  modalCancel: any;
  cancel() {
    if (this.loading || this.submitting) return;
    if (this.is_dirty()) {
      this.modalCancel = this.modalSvc.open(CancellationComponent);
      this.modalCancel.componentInstance.back_path = "hide modal";
      this.modalCancel.componentInstance.back_dismiss = true;
      this.modalCancel.closed.subscribe((res: any) => {
        this.onSubmit();  // yes, save (if valid)
        this.bsModalRef.dismiss();
      });
      this.modalCancel.dismissed.subscribe((_: any) => this.bsModalRef.dismiss());
      return;
    }
    this.bsModalRef.dismiss();
  }

  is_dirty() {
    let dirty = false;
    Object.keys(this.myForm.controls).forEach(key => {
      const field = this.myForm.get(key)!;
      if (field.dirty) { dirty = true; }
    });
    return dirty;
  }

  // ========================================================
  // calc_kelly_perc(): number {
  //   const W = this.calc_winning_prob();
  //   const R = this.calc_winloss_ratio();
  //   if (R === 0) return 0;
  //   return Math.round(1000 * (W - ((1 - W) / R))) / 1000;
  // }

  // calc_winning_prob(): number {
  //   let t = this.myForm.get('transactions')!.value;
  //   // If you don't lose anything, it means win. Exclude exchange fee. 
  //   let numerator = t
  //     .filter((c: any) => c.price_1 !== null) 
  //     .filter((c: any) => c.price_1 >= c.price).length;
  //   return Math.round(numerator / t.length * 100_000) / 100_000;
  // }

  // calc_winloss_ratio(): number {
  //   let t = this.myForm.get('transactions')!.value;
  //   // https://stackoverflow.com/questions/29544371/finding-the-average-of-an-array-using-js
  //   const average = (array: number[]) => array.reduce((a, b) => a + b) / array.length;

  //   // Note, since our amt can be different, we ignore it from the equation. 
  //   let arr_gain = t.filter((c: any) => c.price_1 !== null)
  //     .filter((c: any) => c.price_1 >= c.price)
  //     .map((c: any) => c.price_1 - c.price);
  //   let arr_loss = t.filter((c: any) => c.price_1 !== null)
  //     .filter((c: any) => c.price_1 < c.price)
  //     .map((c: any) => c.price - c.price_1);
    
  //   let avg_gain = arr_gain.length > 0 ? average(arr_gain) : 0;
  //   let avg_loss = arr_loss.length > 0 ? average(arr_loss) : 0;
  //   if (avg_loss == 0) return 0;
  //   return Math.round(avg_gain / avg_loss * 100_000) / 100_000;
  // }

  // calc_brier_score(): number {
  //   let t = this.myForm.get('transactions')!.value;
  //   let summed_value = t.map((c: any) => {
  //     if (c.pred_prob === null || c.price_1 === null || c.amt_1 === null) return 0;
  //     let o_t = c.price_1 >= c.price ? 1 : 0;
  //     return Math.round(((c.pred_prob - o_t) ** 2) * 100_000) / 100_000
  //   }).reduce((partialSum: any, a: any) => partialSum + a, 0);
  //   let N = t.filter((c: any) => c.pred_prob !== null && c.price_1 !== null && c.amt_1 !== null).length;
  //   return Math.round(summed_value / N * 100_000) / 100_000;
  // }

  // calc_row_total(rowNo: number): number {
  //   let tr = this.myForm.get('transactions')!.value;
  //   if (rowNo >= tr.length) { this.doErr("calc_row_total rowNo exceed t.length"); return 0; }
  //   let t = tr[rowNo];
  //   let buy = t.buy ? -1 : 1;  // buy -1, sell 1. 
  //   let value_1 = this.round_to(buy * t.price * t.amt);
  //   if (t.price_1 === null || t.amt_1 === null) return value_1;
  //   let value_2 = this.round_to(1 * t.price_1 * t.amt_1);  // always sell. 
  //   return this.round_to(value_1 + value_2);
  // }

  // ===========================================================
  doErr(err: any) {
    console.error(err);
    this.toastr.error(err);
  }

  // round_to(value: number, dp: number = 5) {
  //   return Math.round(value * 10**dp) / 10**dp;
  // }

  // get errors() {
  //   var myerrors: any = {};
  //   Object.keys(this.myForm.controls).forEach(key => {
  //     // Get errors of every form control
  //     var form = this.myForm.get(key)!;
  //     if (form.errors != null && (form.dirty || form.touched)) { 
  //       myerrors[key] = form.errors; 
  //     }
  //   });

  //   // For master details
  //   let dtls = this.myForm.get('transactions') as FormArray;
  //   dtls.controls.forEach(formgroup => {
  //     // is a form group; we already know. 
  //     var elem = formgroup as FormGroup;
  //     Object.keys(elem.controls).forEach((key) => {
  //       var field = elem.get(key)!;
  //       if (field.errors != null && (field.dirty || field.touched)) {
  //         if (key != 'markup') {
  //           myerrors[key] = field.errors;
  //         } else {
  //           // We change keyname cause repeated. 
  //           myerrors['md_markup'] = field.errors;
  //         }
  //       }
  //     });
  //   });

  //   return Object.keys(myerrors).length ? myerrors : null;
  // }
}
