use failure::Error;
use rustwide::cmd::Command;
use std::fs;

mod utils;

#[test]
fn run_binary_with_same_name_as_file() -> Result<(), Error> {
    let workspace = crate::utils::init_workspace()?;

    let tmpdir = tempfile::tempdir()?;
    std::env::set_current_dir(&tmpdir)?;
    fs::write("true", b"foobar")?;
    // create workspace
    Command::new(&workspace, "true").run()?;

    Ok(())
}
