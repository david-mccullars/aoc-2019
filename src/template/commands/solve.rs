use std::process::{Command, Stdio};

use crate::template::Day;

pub fn handle(day: Day, release: bool, dhat: bool, submit_part: Option<u8>, quiet: bool) {
    let mut cmd_args = vec!["run".to_string()];

    if quiet {
        cmd_args.push("--quiet".to_string());
    }

    cmd_args.extend(["--bin".to_string(), day.to_string()]);

    if dhat {
        cmd_args.extend([
            "--profile".to_string(),
            "dhat".to_string(),
            "--features".to_string(),
            "dhat-heap".to_string(),
        ]);
    } else if release {
        cmd_args.push("--release".to_string());
    }

    cmd_args.push("--".to_string());

    if let Some(submit_part) = submit_part {
        cmd_args.push("--submit".to_string());
        cmd_args.push(submit_part.to_string());
    }

    let mut cmd = Command::new("cargo")
        .args(&cmd_args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    cmd.wait().unwrap();
}
