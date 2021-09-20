use crate::config::{Config, Handler};
use eyre::{eyre, ContextCompat, Result};
use log::{debug, trace};
use regex::Regex;
use std::process::Command;

pub mod cli;
pub mod config;

fn open_with_default(url: &String) -> Result<()> {
    let success = open_with_script(
        url,
        &Handler {
            pattern: String::from(""),
            script: String::from("_default"),
        },
    )?;
    if !success {
        Err(eyre!("Default handler returned non-zero exit code"))
    } else {
        Ok(())
    }
}

fn open_with_script(url: &String, handler: &Handler) -> Result<bool> {
    debug!("running script {} for URL {}", handler.script, url);
    let script_path = Config::get_script_path(&handler.script)?
        .wrap_err_with(|| format!("Script not found for {}", handler.script))?;

    let mut child = Command::new(script_path)
        .arg(&url)
        .arg(&handler.pattern)
        .spawn()?;
    let exit_status = child.wait()?;
    let success = exit_status.success();
    debug!("{} success: {}", handler.script, success);

    Ok(success)
}

pub fn open_url(url: &String) -> Result<()> {
    let config = Config::from_file()?;
    for handler in config.handlers {
        let re = Regex::new(&handler.pattern)?;
        if re.is_match(url) {
            debug!(
                "found match for {} handled by script {}",
                handler.pattern, handler.script
            );
            return match open_with_script(url, &handler)? {
                true => Ok(()),
                false => open_with_default(url),
            };
        } else {
            trace!("No match for {}, {}", handler.pattern, handler.script);
        }
    }
    open_with_default(url)
}
