import { Component, Input, inject } from '@angular/core';
import { SharedModule } from '../../../shared/shared.module';
import { SharedFormsModule } from '../../../shared/shared-forms.module';
import { FontAwesomeModule } from '@fortawesome/angular-fontawesome';
import { NgbActiveModal, NgbModal } from '@ng-bootstrap/ng-bootstrap';
import { FormArray, FormBuilder, FormGroup, Validators } from '@angular/forms';
import { Subscription, interval } from 'rxjs';
import { Http3Service } from '../../../services/http3.service';
import { ToastrService } from 'ngx-toastr';
import { TranslateService } from '@ngx-translate/core';
import { Routes } from '../../../models/routes';
import { CancellationComponent } from '../../cancellation/cancellation.component';
import { faAdd, faXmark } from '@fortawesome/free-solid-svg-icons';
import { faSave } from '@fortawesome/free-regular-svg-icons';
import { UppercaseDirective } from '../../../directives/uppercase.directive';
import { NumberNodotValidatorDirective } from '../../../directives/number-nodot-validator.directive';

@Component({
  selector: 'app-kelly-proj',
  standalone: true,
  imports: [SharedModule, SharedFormsModule, FontAwesomeModule, UppercaseDirective,
  NumberNodotValidatorDirective],
  templateUrl: './kelly-proj.component.html',
  styleUrl: './kelly-proj.component.scss'
})
export class KellyProjComponent {
  bsModalRef = inject(NgbActiveModal);
  private modalSvc = inject(NgbModal);

  faAdd = faAdd;
  faCross = faXmark;
  faSave = faSave;

  @Input() id: number = -1;
  @Input() curr_stage: number = 0;
  @Input() filename: string = "";
  @Input() t_uuid: string = '';
  @Input() t_ver: number = 0;

  items: any;
  loading: boolean = true;
  submitting: boolean = false;
  public myForm: FormGroup;
  subscription: Subscription;

  constructor(private http3: Http3Service, private fb: FormBuilder,
    private toastr: ToastrService, private translate: TranslateService
  ) {
    this.assign_initial_form();
    setTimeout(() => this.loadData(), 100);

    // Save every 5 minute, if applicable. 
    const source = interval(300_000);
    this.subscription = source.subscribe(_ => this.autoSave());
  }

  private assign_initial_form() {
    this.myForm = this.fb.group({
      transactions: this.fb.array([])
    });
  }

  // ==============================================================
  // Load questions
  async loadData() {
    if (this.id == -1) { 
      this.translate.get(["reminder.IdMinusOne", "reminder.IdMinusOneDesc"], {}).subscribe((res: any) => {
        this.toastr.error(res["reminder.IdMinusOne"], res["reminder.IdMinusOneDesc"]);
        this.bsModalRef.dismiss({ ty: res["reminder.IdMinusOne"] });
      }); return;
    }
    let row = {
      t_uuid: this.t_uuid,
      t_ver: this.t_ver,
      stage_index: this.curr_stage,
      pipeline_index: this.id
    };
    this.http3.send(Routes.PiProj, JSON.stringify(row)).then(async (value: any) => {
      let data = this.http3.json_handler(value);
      this.items = data;

      let row2 = {
        filename: this.filename,
        stage_index: this.curr_stage,
        pipeline_index: this.id
      };
      let answers = await this.http3.send(Routes.R, JSON.stringify(row2));
      let answers_json: any = this.http3.json_handler(answers);
      this.set_transactions(answers_json);
      this.loading = false;
    }).catch(err => { this.doErr(err); this.loading = false; });
  }

  private set_transactions(answers_json: any) {
    const data = answers_json.map((c: any) => {
      c.price = parseFloat(c.price);
      c.amt = parseFloat(c.amt);
      c.price_1 = c.price_1 !== null ? parseFloat(c.price_1) : null;
      c.amt_1 = c.amt_1 !== null ? parseFloat(c.amt_1) : null;
      c.pred_prob = parseFloat(c.pred_prob);
      return c;
    });

    let t = this.myForm.get('transactions') as FormArray;
    // here we use push instead of insert
    data.forEach((datum: any) => {
      t.push(this.fb.group({
        // amt_1 validators we also set beforehand, since we have its value already.
        coin: [datum.coin ?? '', [Validators.required, Validators.minLength(1), Validators.maxLength(10)]],
        buy: [datum.buy ?? true, [Validators.required]],
        price: [datum.price ?? 0, [Validators.required, Validators.min(this.min_threshold)]],
        amt: [datum.amt ?? 0, [Validators.required, Validators.min(this.min_amt)]],
        price_1: [datum.price_1 ?? null, [Validators.min(this.min_threshold)]],
        amt_1: [datum.amt_1 ?? null, [Validators.min(this.min_amt), Validators.max(datum.amt)]],
        pred_prob: [datum.pred_prob ?? 1, [Validators.min(0), Validators.max(1)]]
      }));
    });

    // this.cd.detectChanges();
    // console.log(this.calc_row_total(3));
  }

  // ======================================================================
  // Transactions
  min_threshold = 0.00001;
  min_amt = 0.00000001;  // smaller, because BTC is large value. 
  max_transaction = 100;
  param_latest = { value: this.max_transaction };  // also change at kelly.component! 

  add_transaction() {
    let t = this.myForm.get('transactions') as FormArray;
    t.insert(0, this.fb.group({  // insert to the beginning. 
      coin: ['', [Validators.required, Validators.minLength(1), Validators.maxLength(10)]],
      buy: [true, [Validators.required]], // parsing this requires parseInt() later.
      price: [0, [Validators.required, Validators.min(this.min_threshold)]],
      amt: [0, [Validators.required, Validators.min(this.min_amt)]],
      
      // These aren't required, if purely "sell"
      // sell_1: [{value: -1, disabled: true}, [Validators.min(this.min_threshold)]],
      price_1: [, [Validators.min(this.min_threshold)]],
      amt_1: [, [Validators.min(this.min_amt)]],

      // Prediction probability, for brier score. 
      pred_prob: [1, [Validators.min(0), Validators.max(1)]]
    }));

    // Remove extra. 
    if (t.length > this.max_transaction) t.removeAt(t.length - 1); 
  }

  upd_amt_1_validator(rowNo: number) {
    let min_val = [Validators.min(this.min_amt)];
    if (rowNo >= this.transactions.length) { 
      this.translate.get("kelly.UpdAmt1OOB").subscribe((res: any) => {
        this.doErr(res);
      }); return;
    }
    let t = this.transactions[rowNo];
    let amt = t.get('amt')?.value;
    if (amt === null || amt <= this.min_amt) { t.get('amt_1')?.setValidators(min_val); return; }

    min_val.push(Validators.max(amt));
    t.get('amt_1')?.setValidators(min_val);
    t.updateValueAndValidity();
    return;
  }

  remove_transaction(i: number) {
    let t = this.myForm.get('transactions') as FormArray;
    t.removeAt(i);
  }

  // ======================================================================
  autoSave() {
    if (this.submitting || this.loading || this.myForm.invalid) return;
    this.translate.get('proj.Autosave', {}).subscribe((res: string) => {
      this.toastr.info(res, '', { timeOut: 1000 });
    });
    this.submitting = true;
    const row = {
      filename: this.filename,
      stage_index: this.curr_stage,
      pipeline_index: this.id,
      transactions: this.get_transactions()
    };

    this.http3.send(Routes.REditKelly, JSON.stringify(row)).then((_: any) => {
      this.submitting = false;
    }).catch(err => { this.doErr(err); this.submitting = false; })
  }

  onSubmit() {
    if (this.submitting || this.loading || this.myForm.invalid) return;
    this.submitting = true;
    const row = {
      filename: this.filename,
      stage_index: this.curr_stage,
      pipeline_index: this.id,
      transactions: this.get_transactions()
    };

    this.http3.send(Routes.REditKelly, JSON.stringify(row)).then((value: any) => {
      this.submitting = false;
      this.bsModalRef.close({ ty: this.http3.json_handler(value) });
    }).catch(err => { this.doErr(err); this.submitting = false; })
  }

  private get_transactions() {
    return this.myForm.get('transactions')!.value.map((c: any) => {
      c.buy = (typeof c.buy === 'boolean' && c.buy) || c.buy === "true";
      // all requires string.
      c.price = c.price.toString();
      c.amt = c.amt.toString();
      c.price_1 = c.price_1 ? c.price_1.toString() : null;
      c.amt_1 = c.amt_1 ? c.amt_1.toString() : null;
      c.pred_prob = c.pred_prob.toString();
      return c;
    });
  }

  // ========================================================
  modalCancel: any;
  cancel() {
    if (this.loading || this.submitting) return;
    if (this.is_dirty()) {
      this.modalCancel = this.modalSvc.open(CancellationComponent);
      this.modalCancel.componentInstance.back_path = "hide modal";
      this.modalCancel.componentInstance.back_dismiss = true;
      this.modalCancel.closed.subscribe((_: any) => {
        this.onSubmit();
        this.bsModalRef.dismiss();
      });
      this.modalCancel.dismissed.subscribe((_: any) => this.bsModalRef.dismiss());
      return;
    }
    this.bsModalRef.dismiss();
  }

  clear_data() {
    if (this.submitting || this.loading) return;
    this.modalCancel = this.modalSvc.open(CancellationComponent);
    this.modalCancel.componentInstance.back_path = "hide modal";
    this.modalCancel.componentInstance.back_dismiss = true;
    this.modalCancel.componentInstance.title = 'cancellation.Sure';
    this.modalCancel.closed.subscribe((_: any) => {
      this.submitting = true;
      const row = {
        filename: this.filename,
        stage_index: this.curr_stage,
        pipeline_index: this.id
      };
      this.http3.send(Routes.RDelKelly, JSON.stringify(row)).then((value: any) => {
        this.http3.json_handler(value);
        this.translate.get('kelly.ClearData').subscribe((res: string) => {
          this.toastr.success(res);
        });
        this.submitting = false;
        this.reset_form();
      }).catch(err => { this.doErr(err); this.submitting = false; })
    });
  }

  private reset_form() {
    this.assign_initial_form();
    this.loadData();
  }

  private is_dirty() {
    let dirty = false;
    Object.keys(this.myForm.controls).forEach(key => {
      const field = this.myForm.get(key)!;
      if (field.dirty && field.touched) dirty = true;
    });
    return dirty;
  }

  // =========================================================
  get title() { return this.items?.title ?? 'Untitled'; }
  get transactions() {
    const q = this.myForm.get('transactions') as FormArray;
    return q['controls'];
  }

  doErr(err: any) {
    console.error(err);
    if (typeof(err) === 'string') this.toastr.error(this.translate.instant(err || ''));
    else this.toastr.error(err);
  }

  round_to(value: number, dp: number = 5) {
    return Math.round(value * 10**dp) / 10**dp;
  }

  // ========================================================
  // Calculations

  // Need to think how to refresh this after everything loaded.
  calc_kelly_perc(): number {
    const W = this.calc_winning_prob();
    const R = this.calc_winloss_ratio();
    if (R === 0) return 0;
    return Math.round(1000 * (W - ((1 - W) / R))) / 1000;
  }

  calc_winning_prob(): number {
    let t = this.myForm.get('transactions')!.value;
    // If you don't lose anything, it means win. Exclude exchange fee. 
    let numerator = t
      .filter((c: any) => c.price_1 !== null) 
      .filter((c: any) => c.price_1 >= c.price).length;
    return Math.round(numerator / t.length * 100_000) / 100_000;
  }

  calc_winloss_ratio(): number {
    let t = this.myForm.get('transactions')!.value;
    // https://stackoverflow.com/questions/29544371/finding-the-average-of-an-array-using-js
    const average = (array: number[]) => array.reduce((a, b) => a + b) / array.length;

    // Note, since our amt can be different, we ignore it from the equation. 
    let arr_gain = t.filter((c: any) => c.price_1 !== null)
      .filter((c: any) => c.price_1 >= c.price)
      .map((c: any) => c.price_1 - c.price);
    let arr_loss = t.filter((c: any) => c.price_1 !== null)
      .filter((c: any) => c.price_1 < c.price)
      .map((c: any) => c.price - c.price_1);
    
    let avg_gain = arr_gain.length > 0 ? average(arr_gain) : 0;
    let avg_loss = arr_loss.length > 0 ? average(arr_loss) : 0;
    if (avg_loss == 0) return 0;
    return Math.round(avg_gain / avg_loss * 100_000) / 100_000;
  }

  calc_brier_score(): number {
    let t = this.myForm.get('transactions')!.value;
    let summed_value = t.map((c: any) => {
      if (c.pred_prob === null || c.price_1 === null || c.amt_1 === null) return 0;
      let o_t = c.price_1 >= c.price ? 1 : 0;
      return Math.round(((c.pred_prob - o_t) ** 2) * 100_000) / 100_000
    }).reduce((partialSum: any, a: any) => partialSum + a, 0);
    let N = t.filter((c: any) => c.pred_prob !== null && c.price_1 !== null && c.amt_1 !== null).length;
    return Math.round(summed_value / N * 100_000) / 100_000;
  }

  calc_row_total(rowNo: number): number {
    let tr = this.myForm.get('transactions')!.value;
    if (rowNo >= tr.length) { 
      this.translate.get('kelly.CalcRowTotExceed').subscribe((res: string) => {
        this.doErr(res);
      }); return 0;
    }
    let t = tr[rowNo];
    let buy = ((typeof t.buy == "boolean" && t.buy) || t.buy === "true") ? -1 : 1;  // buy -1, sell 1. 
    let value_1 = this.round_to(buy * t.price * t.amt);
    if (t.price_1 === null || t.amt_1 === null) return value_1;
    let value_2 = this.round_to(1 * t.price_1 * t.amt_1);  // always sell. 
    return this.round_to(value_1 + value_2);
  }
}
