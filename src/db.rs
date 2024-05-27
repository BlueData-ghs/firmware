use std::{
    process::{exit, Command},
    str,
};

use anyhow::{Context, Result};
use color_print::cprintln;

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
