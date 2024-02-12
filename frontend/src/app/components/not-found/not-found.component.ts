import { AfterViewInit, Component } from '@angular/core';
import { SharedModule } from '../../shared/shared.module';
import { IconDefinition, faExclamationCircle, faHouseChimney } from '@fortawesome/free-solid-svg-icons';

@Component({
  selector: 'app-not-found',
  standalone: true,
  imports: [SharedModule],
  templateUrl: './not-found.component.html',
  styleUrl: './not-found.component.scss'
})
export class NotFoundComponent {
  faExcCirc = faExclamationCircle;
  faHouse = faHouseChimney;

  constructor() {}
}
