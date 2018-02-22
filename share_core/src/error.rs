use reqwest;
use std::io;

#[derive(Debug)]
pub enum ExecutionError {
  ReqwestError(reqwest::Error),
  IOError(io::Error),
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
