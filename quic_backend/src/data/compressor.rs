/// Compression will first use jsonm to make the json smaller. 
/// Then, it'll compress with Zlib Compressor. 
/// 
/// Alternatively, one could modify the code to use Gz Compressor, or
/// pure Deflate. 

// TBD: All .expect should be replaced with match and error!(...) and stop execution. 

// use anyhow::Error;
use jsonm::{packer::{PackOptions, Packer}, unpacker::{Unpacker, UnpackerError}};
use flate2::{Compression, write::ZlibEncoder, read::ZlibDecoder};
// use uuid::Uuid;

use crate::*;

use self::messages::RD_CANNOT_FIND_FILE;

/// data: the data in json format. 
/// filepath: the path to save this (e.g. ./data/template) in PathBuf. 
/// filename: the filename WITH UUID AND .json.zl already. 
pub(crate) fn compress_and_save(data: String, filepath: PathBuf, filename: String) -> Result<String, String> {
    let path = filepath.as_path();
    let fullpath = path.join(filename);
    compress_and_save_fullpath(data, fullpath)
}

pub(crate) fn compress_and_save_fullpath(data: String, fullpath: PathBuf) -> Result<String, String> {
    let input = data.as_str();

    let mut packer = Packer::new();
    let options = PackOptions::new();
    let packed = packer.pack_string(input, &options);
    if packed.is_err() { return Err("compress_and_save: packing error.".to_string()); }
    let packed = packed.unwrap();

    let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
    let _ = enc.write_all(packed.to_string().as_bytes());
    let compressed_bytes = enc.finish();
    if compressed_bytes.is_err() { return Err("compress_and_save: flate2 compression failed.".to_string()); }
    let compressed_bytes = compressed_bytes.unwrap();

    let output = File::create(fullpath);
    if output.is_err() { return Err("compress_and_save create file failed.".to_string()); }
    let mut output = output.unwrap();
    let c = output.write_all(&compressed_bytes);
    if c.is_err() { return Err("compress_and_save: write file failed.".to_string()); }

    Ok("Successful".to_owned())
}


/// filepath: the path to this saved file
/// filename: the filename INCLUDING UUID AND WITH .json.zl. 
/// 
/// All searching for real filename from uuid, etc., should be done in another
/// function, NOT HERE. 
pub(crate) fn retrieve_decompress(filepath: PathBuf, filename: String) -> Result<Value, String> {
    let path = filepath.as_path();
    let fullpath = path.join(filename);
    retrieve_decompress_fullpath(fullpath)
}

pub(crate) fn retrieve_decompress_fullpath(fullpath: PathBuf) -> Result<Value, String> {
    let contents = fs::read(fullpath);
    if contents.is_err() { return Err(RD_CANNOT_FIND_FILE.to_string()); }
    let contents: &[u8] = &contents.unwrap();

    let mut dec = ZlibDecoder::new(contents);
    let mut packed = String::new();
    let ret = dec.read_to_string(&mut packed);
    if ret.is_err() { return Err("retrieve_decompress: failed to decompress.".to_owned()); }
    // let ret = ret.unwrap();
    // dec.read_to_string(&mut packed).unwrap();

    let mut unpacker = Unpacker::new();
    let v: Result<Value, serde_json::Error> = serde_json::from_str(&packed);
    if v.is_err() { return Err("retrieve_decompress: cannot convert packed into json value.".to_string()); }
    let v: Value = v.unwrap();
    // info!("{:#?}", v);
    
    // Cannot unpack empty json, so we'll do it manually. 
    if v == empty_packed() { return Ok(json!({})); }
    // info!("{:#?}", v.clone());
    let unpacked: Result<Value, UnpackerError> = unpacker.unpack(&v);
    if unpacked.is_err() { error!("{:#?}", v.clone()); error!("{:?}", unpacked.err()); return Err("retrieve_decompress: fail to unpack.".to_string()); }

    Ok(unpacked.unwrap())
}


fn empty_packed() -> Value {
  let mut packer = Packer::new();
  let options = PackOptions::new();
  let packed = packer.pack(&json!({}), &options).unwrap();
  return packed;
}