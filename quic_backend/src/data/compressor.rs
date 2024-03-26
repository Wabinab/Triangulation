/// Compression will first use jsonm to make the json smaller. 
/// Then, it'll compress with Zlib Compressor. 
/// 
/// Alternatively, one could modify the code to use Gz Compressor, or
/// pure Deflate. 

use jsonm::{packer::{PackOptions, Packer}, unpacker::{Unpacker, UnpackerError}};
use flate2::{Compression, write::ZlibEncoder, read::ZlibDecoder};
// use uuid::Uuid;

use crate::{messages::{COMPRESS_CFILE, COMPRESS_FLATE2, COMPRESS_PACKING, COMPRESS_WFILE, RD_CONVJSON, RD_DECOMPRESS, RD_UNPACKING}, *};

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
    if packed.is_err() { error!("compress_and_save packer"); return Err(COMPRESS_PACKING.to_string()); }
    let packed = packed.unwrap();

    let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
    let _ = enc.write_all(packed.to_string().as_bytes());
    let compressed_bytes = enc.finish();
    if compressed_bytes.is_err() { error!("compress_and_save compressor"); return Err(COMPRESS_FLATE2.to_string()); }
    let compressed_bytes = compressed_bytes.unwrap();

    let output = File::create(fullpath);
    if output.is_err() { error!("compress_and_save create file"); return Err(COMPRESS_CFILE.to_string()); }
    let mut output = output.unwrap();
    let c = output.write_all(&compressed_bytes);
    if c.is_err() { error!("compress_and_save write file"); return Err(COMPRESS_WFILE.to_string()); }

    Ok("Successful".to_owned())  // This is never used. 
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
    if contents.is_err() { error!("retrieve_decompress can't find file."); return Err(RD_CANNOT_FIND_FILE.to_string()); }
    let contents: &[u8] = &contents.unwrap();

    let mut dec = ZlibDecoder::new(contents);
    let mut packed = String::new();
    let ret = dec.read_to_string(&mut packed);
    if ret.is_err() { error!("retrieve_decompress decompress failed."); return Err(RD_DECOMPRESS.to_owned()); }

    let v: Result<Value, serde_json::Error> = serde_json::from_str(&packed);
    if v.is_err() { error!("retrieve_decompress convert pack failed."); return Err(RD_CONVJSON.to_string()); }
    let v: Value = v.unwrap();
    
    // Cannot unpack empty json, so we'll do it manually. 
    if v == empty_packed() { info!("empty_packed json; we manually unpacked."); return Ok(json!({})); }

    let mut unpacker = Unpacker::new();
    let unpacked: Result<Value, UnpackerError> = unpacker.unpack(&v);
    if unpacked.is_err() { error!("{:#?}\n{:?}", v.clone(), unpacked.err()); return Err(RD_UNPACKING.to_string()); }

    Ok(unpacked.unwrap())
}


fn empty_packed() -> Value {
  let mut packer = Packer::new();
  let options = PackOptions::new();
  let packed = packer.pack(&json!({}), &options).unwrap();
  return packed;
}