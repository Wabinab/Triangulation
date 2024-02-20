import { AfterViewInit, Component } from '@angular/core';
import { SharedModule } from '../../shared/shared.module';
import { IconDefinition, faExclamationCircle, faHouseChimney } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeModule } from '@fortawesome/angular-fontawesome';

@Component({
  selector: 'app-not-found',
  standalone: true,
  imports: [SharedModule, FontAwesomeModule],
  templateUrl: './not-found.component.html',
  styleUrl: './not-found.component.scss'
})
export class NotFoundComponent {
  faExcCirc = faExclamationCircle;
  faHouse = faHouseChimney;

  constructor() {}
}
