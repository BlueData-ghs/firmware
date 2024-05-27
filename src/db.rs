use std::{
    fs::{self, File},
    os,
    path::{Path, PathBuf},
    process::{exit, Command},
    str,
};

use anyhow::{Context, Result};
use color_print::{cformat, cprintln};
use dialoguer::Confirm;

pub fn verify_drive() -> Result<()> {
    // Using lsblk check to make sure that drive is mounted
    let cmd = Command::new("lsblk")
        .output()
        .context("Failed to run lsblk")?;
    let lsblk_output =
        str::from_utf8(&cmd.stdout).context("Failed to read UTF8 output from running lsblk")?;
    if !lsblk_output.contains("/media/pi/bluedata") {
        cprintln!("<red>Drive for storing data is not connected. Please connect drive and rerun program.</red>");
        exit(1);
    }
    cprintln!("Interface for recording data is <green>mounted and ready</green>");
    Ok(())
}

pub fn create_csv_file(team_number: &u32) -> Result<PathBuf> {
    let path = Path::new("/media/pi/bluedata").join(format!("team-{}.csv", team_number));
    if path.exists() {
        println!();
        let confirmation = Confirm::new()
            .with_prompt(cformat!(
                "<red>CSV file seems to already exist! Is team {} going again?</red>",
                team_number
            ))
            .interact()
            .context("Failed to ask user to confirm redo of team data")?;
        if !confirmation {
            cprintln!("<red>Please rerun program</red>");
        }
    }
    File::create(&path).context("Failed to create team file")?;
    Ok(path)
}
