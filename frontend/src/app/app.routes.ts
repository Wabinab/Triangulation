import { Routes } from '@angular/router';
import { NotFoundComponent } from './components/not-found/not-found.component';
import { HomeComponent } from './components/home/home.component';

export const routes: Routes = [
    { path: '', component: HomeComponent, title: 'Triangulation' },
    { path: '**', component: NotFoundComponent, title: 'Page Not Found' }
];
