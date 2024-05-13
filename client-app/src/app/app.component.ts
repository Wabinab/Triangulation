import { Component } from '@angular/core';
import { RouterOutlet } from '@angular/router';
import { SharedModule } from './shared/shared.module';
import { TranslateService } from '@ngx-translate/core';
import { ToastrService } from 'ngx-toastr';
import { faGear, faGlobe, faMoon } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeModule } from '@fortawesome/angular-fontawesome';
import { ThemeManager } from './services/theme-manager.service';
import { faSun } from '@fortawesome/free-regular-svg-icons';
import { TranslateManager } from './services/translate-manager.service';

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [RouterOutlet, SharedModule, FontAwesomeModule],
  templateUrl: './app.component.html',
  styleUrl: './app.component.scss'
})
export class AppComponent {
  faSettings = faGear;
  faGlobe = faGlobe;

  constructor(public translate: TranslateService, private toastr: ToastrService,
    private themeSvc: ThemeManager, private translateMgr: TranslateManager
  ) {
    this.themeSvc.set_initial_mode();
    translateMgr.set_initial();
    // translate.addLangs(['en', 'fr']);
    // translate.setDefaultLang('en');

    // const browserLang = translate.getBrowserLang() ?? 'en';
    // translate.use(browserLang.match(/en|fr/) ? browserLang : 'en');
  }

  // showSuccess() {
  //   this.toastr.success("Hello world!", "Toastr fun!");
  // }

  closeMenu() {
    if (document.getElementById('navButton')?.getAttribute('aria-expanded') === 'true') {
      document.getElementById('navButton')?.click();
    }
  }

  // ==============================================
  isExpanded = false;

  collapse() {
    this.isExpanded = false;
  }

  toggle() {
    this.isExpanded = !this.isExpanded;
  }

  toggle_light_dark() {
    this.themeSvc.toggle_mode();
  }

  get mode_icon() {
    return this.themeSvc.get_curr_mode() == 'light' ? faSun : faMoon;
  }

  // ==============================================
  use_lang(lang: string) {
    this.translateMgr.use_lang(lang);
  }
}
