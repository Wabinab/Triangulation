import { Injectable } from '@angular/core';
import { ToastrService } from 'ngx-toastr';

@Injectable({
  providedIn: 'root'
})
export class ThemeManager {

  // All of these need to add to angular.json as lazy-loaded items. 
  // themes = [
  //   {
  //     id: 1,
  //     name: 'Default',
  //     href: 'styles.css'
  //   },
  //   {
  //     id: 2,
  //     name: 'Orange',
  //     href: 'orange.css'
  //   },
  //   {
  //     id: 3,
  //     name: 'Green', 
  //     href: 'green.css'
  //   },
  //   {
  //     id: 4,
  //     name: 'Purple',
  //     href: 'purple.css'
  //   }
  // ];

  constructor() { }

  // TBD: Implement save theme to localStorage later. 

  // setTheme(id: number) {
  //   const theme = this.themes.find(theme => theme.id == id);
  //   if (!theme) { 
  //     this.toast.error(`Cannot find theme with id = ${id}`, "Failed", { 
  //       timeOut: 1000 
  //     }); 
  //     return;
  //   }

  //   // console.log(id, theme.href);
  //   document.head.querySelector(`link[rel="stylesheet"]`)!.setAttribute('href', theme.href);
  //   // console.log(document.head.querySelector(`link[rel="stylesheet"]`));
  // }

  // Change between dark and light mode
  dark_light_key = "data-bs-theme";
  set_initial_mode() {
    const curr_mode = localStorage.getItem(this.dark_light_key) ?? 'light';
    document.documentElement.setAttribute('data-bs-theme', curr_mode);
    return curr_mode;
  }

  toggle_mode(): string {
    const curr_mode = document.documentElement.getAttribute('data-bs-theme');
    const next_mode = curr_mode == 'dark' ? 'light' : 'dark';
    document.documentElement.setAttribute('data-bs-theme', next_mode);
    localStorage.setItem(this.dark_light_key, next_mode);
    return next_mode;
  }

  get_curr_mode(): string {
    let item = localStorage.getItem(this.dark_light_key);
    if (item != null && item != undefined && item != '') {
      return item;
    }
    if (typeof document !== 'undefined') {
      return document.documentElement.getAttribute('data-bs-theme') ?? 'light';
    }
    return 'light';
  }
}
