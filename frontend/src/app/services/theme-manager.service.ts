import { Injectable } from '@angular/core';
import { ToastrService } from 'ngx-toastr';

@Injectable({
  providedIn: 'root'
})
export class ThemeManager {

  // All of these need to add to angular.json as lazy-loaded items. 
  themes = [
    {
      id: 1,
      name: 'Default',
      href: 'styles.css'
    },
    {
      id: 2,
      name: 'Orange',
      href: 'orange.css'
    }
  ];

  constructor(private toast: ToastrService) { }

  // TBD: Implement save theme to localStorage later. 

  setTheme(id: number) {
    const theme = this.themes.find(theme => theme.id == id);
    if (!theme) { 
      this.toast.error(`Cannot find theme with id = ${id}`, "Failed", { 
        timeOut: 1000 
      }); 
      return;
    }

    // console.log(id, theme.href);
    document.head.querySelector(`link[rel="stylesheet"]`)!.setAttribute('href', theme.href);
    // console.log(document.head.querySelector(`link[rel="stylesheet"]`));
  }
}
