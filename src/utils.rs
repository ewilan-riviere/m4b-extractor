use anyhow::{bail, Result};
use regex::Regex;

pub fn check_commands(commands: &[&str]) -> Result<()> {
    for &cmd in commands {
        if which::which(cmd).is_err() {
            bail!("Command '{}' not found in PATH", cmd);
        }
    }
    Ok(())
}

pub fn sanitize_filename(name: &str) -> String {
    let re = Regex::new(r#"[^A-Za-z0-9 _-]"#).unwrap();
    let safe = re.replace_all(name, "_");
    let re_multi = Regex::new(r#"_+"#).unwrap();
    re_multi.replace_all(&safe, "_").to_string()
}
