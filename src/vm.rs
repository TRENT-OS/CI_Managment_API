//
// Copyright (C) 2024, HENSOLDT Cyber GmbH
// 
// SPDX-License-Identifier: GPL-2.0-or-later
//
// For commercial licensing, contact: info.cyber@hensoldt.net
//


use std::fs;
use std::path;
use std::io;
use strum_macros::AsRefStr;
use std::{env, sync::LazyLock};

use rocket::tokio::time::{sleep, Duration};



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
#[allow(non_camel_case_types)]
pub enum Command {
    start,
    stop,
    createsnap,
    revert,
}



//------------------------------------------------------------------------------
// Utility Functions
//------------------------------------------------------------------------------


async fn touch(vm: String, command: Command) -> io::Result<()> {

    let path = get_path(&vm, command);

    sleep(Duration::from_secs(30)).await;

    println!("Creating vm command file at: {:?}", &path);
    fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&path)?;
    Ok(())
}


fn get_path(vm: &str, command: Command) -> path::PathBuf {
    let dir: &str = COMMAND_DIR.as_ref();
    return path::Path::new(&dir).join(format!("{}.{}", vm, command.as_ref()));
}



//------------------------------------------------------------------------------
// VM CONTROL FUNCTIONS
//------------------------------------------------------------------------------

pub async fn exec_command(vm: &str, command: Command) {
    rocket::tokio::spawn(touch(vm.to_string(), command));
}

pub async fn start(runner: &str) {
    exec_command(runner, Command::start).await;
}


pub async fn stop(runner: &str) {
    exec_command(runner, Command::stop).await;
}

pub async fn snapshot(runner: &str) {
    exec_command(runner, Command::createsnap).await;
}

pub async fn reset(runner: &str) {
    exec_command(runner, Command::revert).await;
}