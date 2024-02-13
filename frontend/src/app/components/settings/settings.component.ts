import { Component, OnInit } from '@angular/core';
import { ThemeManager } from '../../services/theme-manager.service';
import { ToastrService } from 'ngx-toastr';
import { SharedModule } from '../../shared/shared.module';

@Component({
  selector: 'app-settings',
  standalone: true,
  imports: [SharedModule],
  templateUrl: './settings.component.html',
  styleUrl: './settings.component.scss'
})
export class SettingsComponent implements OnInit {
  themes: any[] = this.themeSvc.themes;
  constructor(private themeSvc: ThemeManager, private toast: ToastrService) {}

  ngOnInit(): void {
    // this.themes = this.themeSvc.themes;
    // console.log(this.themes);
  }

  change_theme(target: any) {
    this.themeSvc.setTheme(target.value);
    // this.toast.info(this.themes.find(c => c.id == target.value).name, "Theme");
  }
}
