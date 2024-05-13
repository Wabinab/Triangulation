import { Injectable } from '@angular/core';
import { TranslateService } from '@ngx-translate/core';

@Injectable({
  providedIn: 'root'
})
export class TranslateManager {

  constructor(private translate: TranslateService) { }

  lang_key = "curr-lang";
  set_initial() {
    this.translate.addLangs(['en', 'fr']);
    this.translate.setDefaultLang('en');

    const setLang = localStorage.getItem(this.lang_key);
    const browserLang = this.translate.getBrowserLang() ?? 'en';
    const backupLang = browserLang.match(/en|fr/) ? browserLang : 'en';
    this.translate.use(setLang ? setLang : backupLang);
  }

  use_lang(lang: string) {
    this.translate.use(lang);
    localStorage.setItem(this.lang_key, lang);
  }
}
