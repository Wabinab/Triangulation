import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';

@Injectable({
  providedIn: 'root'
})
export class GithubService {
  baseUrl: string = "https://raw.githubusercontent.com/Wabinab/Triangulation_Sample/main"
  constructor(private http: HttpClient) { }

  get_index(): Observable<any> {
    return this.http.get(`${this.baseUrl}/index.json`, { responseType: 'json' });
  }

  // download_object(filename: string) {
  //   // return this.http.get(`https://github.com/Wabinab/Triangulation_Sample/raw/main/${filename}`);
  //   const link = document.createElement("a");
  //   link.href = `https://github.com/Wabinab/Triangulation_Sample/raw/main/${filename}`;
  //   link.download = filename;
  //   link.click();
  // }
}
