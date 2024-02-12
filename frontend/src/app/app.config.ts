import { ApplicationConfig, importProvidersFrom } from '@angular/core';
import { provideRouter } from '@angular/router';

import { routes } from './app.routes';
import { provideClientHydration } from '@angular/platform-browser';
import { HttpClient, provideHttpClient, withFetch } from '@angular/common/http';
import { TranslateLoader, TranslateModule } from '@ngx-translate/core';
import { TranslateHttpLoader } from '@ngx-translate/http-loader';
import { ToastNoAnimationModule, provideToastr } from 'ngx-toastr';

export function HttpLoaderFactory(httpClient: HttpClient) {
  return new TranslateHttpLoader(httpClient);
}

export const appConfig: ApplicationConfig = {
  providers: [provideRouter(routes), provideClientHydration(),
    provideHttpClient(withFetch()), 
    importProvidersFrom(TranslateModule.forRoot({
      loader: {
        provide: TranslateLoader, useFactory: HttpLoaderFactory, 
        deps: [HttpClient]
      },
    })), 
    provideToastr(),
    importProvidersFrom(ToastNoAnimationModule.forRoot()), 
  ]
};
