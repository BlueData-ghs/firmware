use std::{
    fs::File,
    path::Path,
    process::{exit, Command},
    str,
};

use anyhow::{Context, Result};
use color_print::{cformat, cprintln};
use csv::{Writer, WriterBuilder};
use dialoguer::Confirm;

use crate::sensors::Measurement;

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

pub fn create_csv_file(team_number: &u32) -> Result<Writer<File>> {
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
    Ok(WriterBuilder::new()
        .from_path(path)
        .context("Failed to create writer builder for CSV file")?)
}

pub fn write_measurement(csv_file: &mut Writer<File>, measurement: Measurement) -> Result<()> {
    csv_file
        .write_record(&[
            measurement.depth.to_string(),
            measurement.temp.to_string(),
            measurement.salinity.to_string(),
            measurement.time.to_string(),
        ])
        .context("Failed to write data to CSV file")?;
    println!();
    cprintln!("<green>Successfully recorded the following data:</green>");
    cprintln!("\t1. <blue>Depth</blue> {}m", measurement.depth);
    cprintln!("\t2. <yellow>Temperature</yellow> {} °C", measurement.temp);
    cprintln!(
        "\t3. <magenta>Light</magenta> {}%",
        measurement.light * 100.0
    );
    cprintln!("\t4. <cyan>Salinity</cyan> {} g/L", measurement.salinity);
    cprintln!("\t5. Time {}", measurement.time.format("%x %r"));
    Ok(())
}
