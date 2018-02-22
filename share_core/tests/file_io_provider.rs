extern crate share_core;
extern crate reqwest;

use share_core::provider::Provider;
use share_core::providers::file_io_provider::FileIOProvider;

#[test]
fn it_uploads() {
  let path = String::from("/home/edouard/dev/share/share_core/tests/resources/plaintext.txt");

  match FileIOProvider::upload(&path) {
    Ok(_) => {}
    Err(_) => panic!("File was not uploaded")
  }
}
