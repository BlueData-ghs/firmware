use std::{io, process::exit};

use anyhow::{Context, Result};
use color_print::cprintln;

/// Ask the user the ream number that they are apart of
pub fn team_number() -> Result<u32> {
    println!("\nBefore we get started what team number are you?");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .context("Failed to ask for team number by reading line from buffer")?;
    Ok(input
        .trim()
        .parse::<u32>()
        .context("Failed to parse number from input")?)
}

/// Ask the user the depth of the current measurements
///
/// If the user is done measuring, they will input "done" and the program will exit with a status code of 0
pub fn depth() -> Result<u32> {
    println!("\nCurrent depth in meters?");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .context("Failed to ask for depth by reading line from buffer")?;

    if input.to_lowercase().contains("done") {
        cprintln!(
            "\n\n<yellow>Done measuring, thank you for using <blue>BlueData</blue>!</yellow>"
        );
        exit(0);
    }

    Ok(input
        .trim()
        .parse::<u32>()
        .context("Failed to parse number from input")?)
}
