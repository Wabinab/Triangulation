import { Routes } from '@angular/router';
import { NotFoundComponent } from './components/not-found/not-found.component';

export const routes: Routes = [
  { pathMatch: "full", path: "", loadComponent: () => import(
      "./components/home/home.component"
  ).then((m) => m.HomeComponent), title: 'Triangulation'},
  // { path: "settings", loadComponent: () => import(
  //     "./components/settings/settings.component"
  // ).then((m) => m.SettingsComponent), title: 'Settings' },
  { path: 'template', loadComponent: () => import(
      "./components/template/template.component"
  ).then((m) => m.TemplateComponent), title: 'Template' },
  { path: 'project', loadComponent: () => import(
      "./components/project/project.component"
  ).then((m) => m.ProjectComponent), title: 'Project'},
  
  { path: '**', loadComponent: () => import(
    "./components/not-found/not-found.component"
  ).then((m) => m.NotFoundComponent), title: 'Page Not Found'},
];
