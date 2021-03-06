#[macro_use]
extern crate serde_derive;
extern crate docopt;
extern crate share_core;
extern crate which;
extern crate yansi;
extern crate spinners;

use share_core::provider::Provider;
use share_core::providers::file_io_provider::FileIOProvider;
use share_core::providers::imgur_provider::ImgurProvider;
use share_core::providers::get_provider_instance;
use yansi::Paint;
use docopt::Docopt;
use std::process::exit;
use std::io;
use std::io::Write;
use std::path::Path;
use spinners::{Spinner, Spinners};

mod clip;

const USAGE: &'static str = "
Share.

Usage:
  share [options] <file>
  share providers
  share (-h | --help)
  share --version

Options:
  -h --help           Show this screen.
  --version           Show version.
  -v --verbose        Enable verbose output.
  -u --unix           Enable Unix mode for nerds who want only raw output.
  -n --no-input       Disable all warnings that might stop the program asking for input.
  --provider=<prvdr>  Override the default provider.
";

#[derive(Debug, Deserialize)]
struct Args {
  arg_file: String,
  arg_provider: Option<String>,
  flag_unix: bool
}

fn clear() {
  io::stdout().write_all("\r".as_bytes()).unwrap();
}

fn main() {
  let args: Args = Docopt::new(USAGE)
      .and_then(|d| d.deserialize())
      .unwrap_or_else(|e| e.exit());

  let unix_mode = args.flag_unix;
  let provider_name = match args.arg_provider {
    Some(provider) => provider,
    None => String::from("")
  };

  let provider = get_provider_instance(provider_name);

  let mut sp: Option<Spinner> = None;

  if !unix_mode {
    sp = Some(Spinner::new(Spinners::Dots9, "Uploading...".into()));
  }

  let response = provider.upload(Path::new(&args.arg_file));

  match response {
    Ok(response) => {
      if let Some(sp) = sp {
        sp.stop();
      }

      clip::Clipboard::copy(&response.link);
      clear();

      if unix_mode {
        println!("{}", &response.link.to_owned());
      }
      else {
        println!("Uploaded file {} to {}.", Paint::green(&args.arg_file), Paint::red("File.IO"));
        println!("You can access the file via {}.", Paint::blue(&response.link.to_owned())
            .underline());
      }
    }
    Err(err) => {
      if !unix_mode {
        println!("{:?}", err);
      }

      exit(1);
    }
  }
}
