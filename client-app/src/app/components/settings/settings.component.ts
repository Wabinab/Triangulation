import { Component, OnInit } from '@angular/core';
import { ThemeManager } from '../../services/theme-manager.service';
import { ToastrService } from 'ngx-toastr';
import { SharedModule } from '../../shared/shared.module';
import { faSun } from '@fortawesome/free-regular-svg-icons';
import { faMoon } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeModule } from '@fortawesome/angular-fontawesome';

@Component({
  selector: 'app-settings',
  standalone: true,
  imports: [SharedModule, FontAwesomeModule],
  templateUrl: './settings.component.html',
  styleUrl: './settings.component.scss'
})
export class SettingsComponent implements OnInit {
  // themes: any[] = this.themeSvc.themes;
  curr_mode: string = '';
  constructor(private themeSvc: ThemeManager, private toast: ToastrService) {}

  ngOnInit(): void {
    // this.themes = this.themeSvc.themes;
    // console.log(this.themes);
    this.curr_mode = this.themeSvc.get_curr_mode();
  }

  // change_theme(target: any) {
  //   this.themeSvc.setTheme(target.value);
  //   // this.toast.info(this.themes.find(c => c.id == target.value).name, "Theme");
  // }

  toggle_light_dark() {
    this.curr_mode = this.themeSvc.toggle_mode();
  }

  get mode_icon() {
    return this.curr_mode == 'light' ? faSun : faMoon;
  }
}
