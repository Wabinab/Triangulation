/// Compression will first use jsonm to make the json smaller. 
/// Then, it'll compress with Zlib Compressor. 
/// 
/// Alternatively, one could modify the code to use Gz Compressor, or
/// pure Deflate. 

use std::path::PathBuf;
use jsonm::{packer::{PackOptions, Packer}, unpacker::Unpacker};
use flate2::{Compression, write::ZlibEncoder, read::ZlibDecoder};
use uuid::Uuid;

use crate::*;

/// data: the data in json format. 
/// filepath: the path to save this (e.g. ./data/template) in PathBuf. 
/// filename: the filename WITHOUT UUID AND WITHOUT .json.zl.
pub(crate) fn compress_and_save(data: String, filepath: PathBuf, filename: String) {
    let input = data.as_str();
    let path = filepath.as_path();

    let mut filename = filename;
    let uuid = Uuid::now_v7().to_string();
    filename.push_str(uuid.as_str());
    filename.push_str(".json.zl");

    let mut packer = Packer::new();
    let options = PackOptions::new();
    let packed = packer.pack_string(input, &options)
        .expect("compress_and_save packing error.");

    let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
    let _ = enc.write_all(packed.to_string().as_bytes());
    let compressed_bytes = enc.finish().expect("compress_and_save flate2 compression failed.");

    let mut output = File::create(path.join(filename))
        .expect("compress_and_save create file failed.");
    output.write_all(&compressed_bytes).expect("compress_and_save write file failed.");
}


/// filepath: the path to this saved file
/// filename: the filename INCLUDING UUID AND WITH .json.zl. 
/// 
/// All searching for real filename from uuid, etc., should be done in another
/// function, NOT HERE. 
pub(crate) fn retrieve_decompress(filepath: PathBuf, filename: String) -> Value {
    let path = filepath.as_path();
    let contents: &[u8] = &fs::read(path.join(filename))
        .expect("retrieve_decompress cannot find file.");

    let mut dec = ZlibDecoder::new(contents);
    let mut packed = String::new();
    dec.read_to_string(&mut packed).unwrap();

    let mut unpacker = Unpacker::new();
    let v: Value = serde_json::from_str(&packed)
        .expect("retrieve_decompress cannot convert packed into json value.");
    let unpacked: Value = unpacker.unpack(&v)
        .expect("retrieve_decompress fail to unpack.");

    unpacked
}