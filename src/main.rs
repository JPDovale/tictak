use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process::Command;

use chrono::Local;
use notify_rust::Notification;

fn read_configuration_file(path: &str) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut lines = Vec::new();

    for lines_result in reader.lines() {
        let line = lines_result?;
        lines.push(line);
    }

    Ok(lines)
}

fn main() -> io::Result<()> {
    let config_file = ".tictak.conf";

    if !Path::new(config_file).exists() {
        File::create(config_file)?;
    }

    let lines = read_configuration_file(config_file)?;

    for line in lines {
        let status_git_add = Command::new("git")
            .arg("add")
            .arg(".")
            .current_dir(&line)
            .status()?;

        if !status_git_add.success() {
            let body = format!("Failed to git add in {}", &line);

            let _ = Notification::new()
                .summary("Failed to git add")
                .body(&body)
                .icon("dialog-error")
                .show();

            return Err(io::Error::new(io::ErrorKind::Other, "Failed to git add"));
        }

        let now = Local::now();
        let message = format!("TicTak: {}", now);

        let status_git_commit = Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg(&message)
            .current_dir(&line)
            .status()?;

        if !status_git_commit.success() {
            let body = format!("Failed to git commit in {}", &line);

            let _ = Notification::new()
                .summary("Failed to git commit")
                .body(&body)
                .icon("dialog-error")
                .show();

            return Err(io::Error::new(io::ErrorKind::Other, "Failed to git commit"));
        }

        let status_git_push = Command::new("git")
            .arg("push")
            .arg("origin")
            .arg("main")
            .current_dir(&line)
            .status()?;

        if !status_git_push.success() {
            let body = format!("Failed to git push in {} | {}", &line, io::ErrorKind::Other);

            let _ = Notification::new()
                .summary("Failed to git commit")
                .body(&body)
                .icon("dialog-information")
                .show();

            return Err(io::Error::new(io::ErrorKind::Other, "Failed to git push"));
        }

        let body = format!("Repo {} updated with commit {}", &line, &message);

        let _ = Notification::new()
            .summary("Done ^^!")
            .body(&body)
            .icon("dialog-success")
            .show();
    }

    Ok(())
}
