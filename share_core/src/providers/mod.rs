pub mod file_io_provider;
pub mod imgur_provider;

use self::imgur_provider::ImgurProvider;
use self::file_io_provider::FileIOProvider;

//pub enum Providers {
//  ImgurProvider,
//  FileIOProvider
//}

use provider::{Provider};

pub fn get_provider_instance(provider: String) -> Box<Provider> {
  match provider.as_ref() {
    "imgur" => Box::new(ImgurProvider),
    _ => Box::new(FileIOProvider)
  }
}
