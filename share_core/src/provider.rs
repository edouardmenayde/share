use std::path::Path;
use error::ExecutionError;

pub const GIGABYTE: u32 = 1000000000;

pub struct ProviderResponse {
  pub link: String
}

pub trait Provider {
  /// Max size of an upload in Bytes
  fn get_max_size(&self) -> u64;
  /// Uploads a file to the provider and return the link atm
  fn upload(&self, filename: &Path) -> Result<ProviderResponse, ExecutionError>;
}
