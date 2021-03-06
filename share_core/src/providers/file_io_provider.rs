use reqwest::{Response, Client, multipart};
use std::fs::{File, metadata};
use std::path::Path;

use error::ExecutionError;

use provider::{Provider, ProviderResponse, GIGABYTE};

pub struct FileIOProvider;

#[derive(Deserialize)]
struct ResponseBody {
  success: bool,
  pub link: String,
//  key: String,
//  expiry: String,
}

impl Provider for FileIOProvider {
  fn get_max_size(&self) -> u64 {
    5 * GIGABYTE as u64
  }

  fn upload(&self, filename: &Path) -> Result<ProviderResponse, ExecutionError> {
    let metadata = metadata(filename)?;
    let file_size = metadata.len();

    if file_size > self.get_max_size() {
      return Err(ExecutionError::Custom(String::from("File exceeds max size.")));
    }

    let client = Client::new();

    let form = multipart::Form::new()
        .file("file", filename)?;

    let mut res = client
        .post("https://file.io")
        .multipart(form)
        .send()?;

    let body: ResponseBody = res.json()?;

    if !body.success {
      return Err(ExecutionError::Custom(String::from("File upload failed.")));
    }

    Ok(ProviderResponse {
      link: body.link
    })
  }
}
