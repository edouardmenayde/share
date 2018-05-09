use reqwest;
use std::io;
extern crate serde_json;

use serde_json::{Value, Error};

#[derive(Debug)]
pub enum ExecutionError {
  ReqwestError(reqwest::Error),
  IOError(io::Error),
  SerdeJsonError(serde_json::Error),
  Custom(String)
}

impl From<reqwest::Error> for ExecutionError {
  fn from(e: reqwest::Error) -> Self {
    ExecutionError::ReqwestError(e)
  }
}

impl From<io::Error> for ExecutionError {
  fn from(e: io::Error) -> Self {
    ExecutionError::IOError(e)
  }
}

impl From<String> for ExecutionError {
  fn from(e: String) -> Self {
    ExecutionError::Custom(e)
  }
}

impl From<serde_json::Error> for ExecutionError {
  fn from(e: serde_json::Error) -> Self {
    ExecutionError::SerdeJsonError(e)
  }
}
