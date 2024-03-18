import { ApplicationConfig, importProvidersFrom } from '@angular/core';
import { PreloadAllModules, provideRouter, withPreloading } from '@angular/router';

import { routes } from './app.routes';
import { provideClientHydration } from '@angular/platform-browser';
import { HttpClient, provideHttpClient, withFetch } from '@angular/common/http';
import { TranslateLoader, TranslateModule } from '@ngx-translate/core';
import { TranslateHttpLoader } from '@ngx-translate/http-loader';
import { ToastNoAnimationModule, provideToastr } from 'ngx-toastr';
import { NgSelectModule } from '@ng-select/ng-select';
// import { provideAnimationsAsync } from '@angular/platform-browser/animations/async';

export function HttpLoaderFactory(httpClient: HttpClient) {
  return new TranslateHttpLoader(httpClient);
}

export const appConfig: ApplicationConfig = {
  providers: [provideRouter(routes, withPreloading(PreloadAllModules)), provideClientHydration(),
    provideHttpClient(withFetch()), 
    importProvidersFrom(TranslateModule.forRoot({
      loader: {
        provide: TranslateLoader, useFactory: HttpLoaderFactory, 
        deps: [HttpClient]
      },
    })), 
    provideToastr(),
    importProvidersFrom(ToastNoAnimationModule.forRoot()), 
    importProvidersFrom(NgSelectModule)
  ]
};
