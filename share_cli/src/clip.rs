// Inspired by https://github.com/conradkdotcom/rooster/blob/9f8cea8befbd2e1bf8e68cbadaade1de86e4ce19/src/clip.rs
use share_core::error::ExecutionError;

pub struct Clipboard;

impl Clipboard {
  #[cfg(any(windows, macos))]
  pub fn copy(text: &String) -> Result<(), ExecutionError> {
    use clipboard::ClipboardProvider;
    use clipboard::ClipboardContext;

    let mut context: ClipboardContext = ClipboardProvider::new().map_err(|_| ())?;
    context.set_contents(text.to_owned()).map_err(|_| ())?;

    Ok(())
  }

  #[cfg(all(unix, not(macos)))]
  pub fn generate_appropriate_command(text: &String) -> Result<String, ExecutionError> {
    use which::which;

    let xsel = which("xsel");
    if xsel.is_ok() {
      return Ok(format!(
        "printf '%s' {} | {} -ib 2> /dev/null",
        text,
        xsel.unwrap().to_string_lossy()
      ));
    }

    let xclip = which("xclip");
    if xclip.is_ok() {
      return Ok(format!(
        "printf '%s' {} | {} -selection clipboard 2> /dev/null",
        text,
        xclip.unwrap().to_string_lossy()
      ));
    }

    Err(ExecutionError::Custom(String::from("Cannot find appropriate command to copy.")))
  }

  #[cfg(all(unix, not(macos)))]
  pub fn copy(text: &String) -> Result<(), ExecutionError> {
    use std::process::Command;

    let shell = Clipboard::generate_appropriate_command(text)?;

    let status = Command::new("sh")
        .args(&["-c", shell.as_str()])
        .status()?;

    if !status.success() {
      return match status.code() {
        Some(code) => {
          Err(ExecutionError::Custom(format!(
            "Could not copy to clipboard, command exited with {}", code
          )))
        }
        None => {
          Err(ExecutionError::Custom(format!("Could not copy to clipboard, command exited
        due to a signal")))
        }
      };
    }

    Ok(())
  }
}
