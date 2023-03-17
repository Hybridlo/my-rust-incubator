use std::borrow::Cow;
use std::env::args;
use std::env::var;
use std::path::Path;

const DEFAULT_PATH: &str = "/etc/app/app.conf";
const CONF_VAR: &str = "APP_CONF";

fn get_path() -> Result<Cow<'static, str>, ()> {
    let default_conf = Path::new(DEFAULT_PATH);

    // get the default
    if default_conf.try_exists().unwrap_or(false) {
        return Ok(DEFAULT_PATH.into());
    }

    // get from environment variable
    if let Ok(var_conf_str) = var(CONF_VAR) {
        let var_conf = Path::new(&var_conf_str);

        if var_conf.try_exists().unwrap_or(false) {
            return Ok(var_conf_str.into());
        }
    }

    // get from cmd argument
    if let Some(arg_conf_str) = args().skip_while(|arg| arg != "--conf").nth(1) {
        let arg_conf = Path::new(&arg_conf_str);

        if arg_conf.try_exists().unwrap_or(false) {
            return Ok(arg_conf_str.into());
        }
    }

    Err(())
}

fn main() {
    println!("{}", get_path().unwrap_or("Error".into()));
}
