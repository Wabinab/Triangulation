import { Component } from '@angular/core';
import { RouterOutlet } from '@angular/router';
import { SharedModule } from './shared/shared.module';
import { TranslateService } from '@ngx-translate/core';
import { ToastrService } from 'ngx-toastr';
import { faGear, faGlobe } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeModule } from '@fortawesome/angular-fontawesome';

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

  constructor(public translate: TranslateService, private toastr: ToastrService) {
    translate.addLangs(['en', 'fr']);
    translate.setDefaultLang('en');

    const browserLang = translate.getBrowserLang() ?? 'en';
    translate.use(browserLang.match(/en|fr/) ? browserLang : 'en');
  }

  showSuccess() {
    this.toastr.success("Hello world!", "Toastr fun!");
  }

  closeMenu() {
    if (document.getElementById('navButton')?.getAttribute('aria-expanded') === 'true') {
      document.getElementById('navButton')?.click();
    }
  }
}
