use std::fs;
use std::path;
use std::io;
use strum_macros::AsRefStr;
use std::{env, sync::LazyLock};



//------------------------------------------------------------------------------
// Config
//------------------------------------------------------------------------------


static COMMAND_DIR: LazyLock<String> = LazyLock::new(|| {
    let default: &str = "/tmp/test";
    env::var("COMMAND_SHARE").unwrap_or(default.to_string())
});



//------------------------------------------------------------------------------
// Data Structures
//------------------------------------------------------------------------------


#[derive(Debug, AsRefStr)]
pub enum Command {
    Start,
    Stop,
    Snapshot,
    Reset,
}



//------------------------------------------------------------------------------
// Utility Functions
//------------------------------------------------------------------------------


fn touch(path: &path::Path) -> io::Result<()> {
    println!("Creating vm command file at: {:?}", path);
    fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)?;
    Ok(())
}


fn get_path(vm: &str, command: Command) -> path::PathBuf {
    let dir: &str = COMMAND_DIR.as_ref();
    return path::Path::new(&dir).join(format!("{}.{}", vm, command.as_ref()));
}



//------------------------------------------------------------------------------
// VM CONTROL FUNCTIONS
//------------------------------------------------------------------------------


pub fn exec_command(vm: &str, command: Command) {
    touch(&get_path(vm, command)).unwrap();
}

pub async fn start(runner: &str) {
    exec_command(runner, Command::Start);
}


pub async fn stop(runner: &str) {
    exec_command(runner, Command::Stop);
}

pub async fn snapshot(runner: &str) {
    exec_command(runner, Command::Snapshot);
}

pub async fn reset(runner: &str) {
    exec_command(runner, Command::Reset);
}