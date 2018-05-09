extern crate share_core;
extern crate reqwest;

use share_core::provider::Provider;
use share_core::providers::file_io_provider::FileIOProvider;
use share_core::providers::imgur_provider::ImgurProvider;
use std::path::Path;

#[test]
fn it_uploads_to_fileio() {
  let path = Path::new("./tests/resources/plaintext.txt");

  match FileIOProvider.upload(&path) {
    Ok(_) => {}
    Err(err) => panic!("File was not uploaded, {:?}", err)
  }
}


#[test]
fn it_uploads_to_imgur() {
  let path = Path::new("./tests/resources/example.png");

  match ImgurProvider.upload(&path) {
    Ok(_) => {}
    Err(err) => panic!("File was not uploaded, {:?}", err)
  }
}
