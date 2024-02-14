import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';

@Injectable({
  providedIn: 'root'
})
export class Http3Service {

  url_host = "https://localhost:4443";
  // url_host = "https://4443-wabinab-triangulation-d6l0sn9rmfn.ws-us108.gitpod.io";
  fingerprint: any;

  constructor(private http: HttpClient) {
    this.http.get(`https://4443-wabinab-triangulation-d6l0sn9rmfn.ws-us108.gitpod.io`, {responseType: 'text'}).subscribe(fHex => {
      this.fingerprint = [];
      for (let c = 0; c < fHex.length - 1; c += 2) {
        this.fingerprint.push(parseInt(fHex.substring(c, c + 2), 16));
      }
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
    console.log(this.fingerprint);
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
