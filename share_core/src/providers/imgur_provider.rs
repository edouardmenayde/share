use reqwest::{Response, Client, multipart};
use reqwest::header::Authorization;
use std::fs::{File, metadata};
use std::path::Path;

use error::ExecutionError;

use provider::{Provider, ProviderResponse, GIGABYTE};

extern crate serde_json;

use serde_json::{Value, Error};

pub struct ImgurProvider;

#[derive(Debug)]
#[derive(Deserialize)]
struct ResponseBodyData {
  pub link: Option<String>,
}

#[derive(Debug)]
#[derive(Deserialize)]
struct ResponseBody {
  pub data: Option<ResponseBodyData>,
  pub success: bool,
}

impl Provider for ImgurProvider {
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
        .file("image", filename)?;

    let mut res = client
        .post("https://api.imgur.com/3/image")
        .header(Authorization("Client-ID 358919b724940a5".to_owned()))
        .multipart(form)
        .send()?;

    let data = res.text()?;

    let body: ResponseBody = serde_json::from_str(data.as_str())?; // Can't use .json of res...

    if !body.success {
      return Err(ExecutionError::Custom(String::from("File upload failed.")));
    }

    if let Some(data) = body.data {
      if let Some(link) = data.link {
        return Ok(ProviderResponse { link });
      }
    }

    Err(ExecutionError::Custom(String::from("File upload failed.")))
  }
}

