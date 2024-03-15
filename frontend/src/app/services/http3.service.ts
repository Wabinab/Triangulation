import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { ToastrService } from 'ngx-toastr';

@Injectable({
  providedIn: 'root'
})
export class Http3Service {

  url_host = "https://localhost:4443";
  cert_host = "assets/localhost.hex"
  fingerprint: any;

  constructor(private http: HttpClient, private toastr: ToastrService) {
    this.fetch_fingerprint();
  }

  fetch_fingerprint() {
    // We'll fix cors later. 
    this.http.get(`${this.cert_host}`, {responseType: 'text'}).subscribe(fHex => {
      this.fingerprint = [];
      for (let c = 0; c < fHex.length - 1; c += 2) {
        this.fingerprint.push(parseInt(fHex.substring(c, c + 2), 16));
      }
      // console.log(this.fingerprint);
    }, err => {
      console.error(err);
      this.toastr.error("Renewing cert. Refreshing in 5 seconds...");
      setTimeout(() => window.location.reload(), 5000);
      // console.log(this.fingerprint);
    });
  }

  /// GET, POST, PUT, DELETE, etc. all in one single function. 
  /// 
  /// `body` preferably JSON-stringified string; but could be normal string too! 
  /// However it's created, you need to fit it in the backend.
  ///
  /// `path` must start with `/`. It cannot be empty.
  async send(path: string, body: string | null, datagram: boolean = false) {
    // No need serverCertificateHashes if you have PKI on server cert. 
    const transport = new WebTransport(`${this.url_host}${path}`, {
      serverCertificateHashes: [{
        "algorithm": "sha-256",
        "value": new Uint8Array(this.fingerprint)
      }]
    });
    // console.log(transport);
    await transport.ready;

    let writer;
    let reader;

    if (!datagram) {
      const stream = await transport.createBidirectionalStream();
      writer = stream.writable.getWriter();
      reader = stream.readable.getReader();
    } else {
      writer = transport.datagrams.writable.getWriter();
      reader = transport.datagrams.readable.getReader();
    }

    if (body) await this.write(writer, body);

    let data = await this.read(reader);
    transport.close();
    return await data;
  }

  /// While this is used in lots of situations, it's not in all situations. 
  /// That's why we separate it from the send() function. 
  /// if suppress_err (suppress error), won't check for "err". 
  json_handler(data: string, suppress_err: boolean = false) {
    let json_data = JSON.parse(data);
    if (!suppress_err && json_data.err && json_data.err.length > 0) {
      throw new Error(json_data.err);
    }
    return json_data;
  }

  // =====================================================================
  private async write(writer: any, msg: string) {
    const encoded = new TextEncoder().encode(msg);
    await writer.write(encoded);
    await writer.close();
    writer.releaseLock();
  }

  private async read(reader: any) {
    const { value } = await reader.read();
    const recv = new TextDecoder().decode(value);
    return recv;
  }
}
