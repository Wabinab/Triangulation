import { Routes } from '@angular/router';
import { NotFoundComponent } from './components/not-found/not-found.component';
import { HomeComponent } from './components/home/home.component';
import { SettingsComponent } from './components/settings/settings.component';
import { TemplateComponent } from './components/template/template.component';

export const routes: Routes = [
    { path: '', component: HomeComponent, title: 'Triangulation' },
    { path: 'settings', component: SettingsComponent, title: 'Settings' },
    { path: 'template', component: TemplateComponent, title: 'Template' },
    { path: '**', component: NotFoundComponent, title: 'Page Not Found' }
];
