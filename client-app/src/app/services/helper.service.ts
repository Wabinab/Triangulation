import { Injectable } from '@angular/core';
import { CardTypes } from '../models/card-types';
import { faBell, faListCheck, faMoneyBillWheat, faQuestion } from '@fortawesome/free-solid-svg-icons';

@Injectable({
  providedIn: 'root'
})
export class HelperService {

  faReminder = faBell;
  faInvestment = faMoneyBillWheat;
  faChecklist = faListCheck;
  faQuestionMark = faQuestion;

  constructor() { }

  public get_icon_by_ty(ty: number) {
    if (ty == CardTypes.Reminders) return this.faReminder;
    if (ty == CardTypes.Kelly) return this.faInvestment;
    if (ty == CardTypes.Checklist) return this.faChecklist;
    return this.faQuestionMark;
  }
}
