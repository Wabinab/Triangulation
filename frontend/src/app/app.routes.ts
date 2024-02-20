import { Routes } from '@angular/router';
import { NotFoundComponent } from './components/not-found/not-found.component';
// import { HomeComponent } from './components/home/home.component';
// import { SettingsComponent } from './components/settings/settings.component';
// import { TemplateComponent } from './components/template/template.component';

export const routes: Routes = [
    // Not lazily loaded. 
    // { path: '', component: HomeComponent, title: 'Triangulation' },

    // Lazily loaded
    { pathMatch: "full", path: "", loadComponent: () => import(
        "./components/home/home.component"
    ).then((m) => m.HomeComponent), title: 'Triangulation'},
    { path: "settings", loadComponent: () => import(
        "./components/settings/settings.component"
    ).then((m) => m.SettingsComponent), title: 'Settings' },
    { path: 'template', loadComponent: () => import(
        "./components/template/template.component"
    ).then((m) => m.TemplateComponent), title: 'Template' },

    { path: '**', component: NotFoundComponent, title: 'Page Not Found' },
];
