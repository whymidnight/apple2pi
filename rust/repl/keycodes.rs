use hex::FromHex;
use indoc::formatdoc;
use repl_rs::{Command, Parameter, Result, Value};
use repl_rs::{Convert, Repl};
use std::collections::HashMap;
use std::str;

fn from_keydown_hex<T>(args: HashMap<String, Value>, _context: &mut T) -> Result<Option<String>> {
    Ok(None)
}

fn main() -> Result<()> {
    let mut repl = Repl::new(())
        .with_name("ascii_finder")
        .with_version("v0.0.0")
        .with_description("get keyup_hex and ascii representation of keycode")
        .add_command(
            Command::new("get", from_keydown_hex)
                .with_parameter(Parameter::new("keydown_hex").set_required(true)?)?
                .with_help("find keyup_hex and ascii representation of keycode"),
        );
    repl.run()
}
