use chrono::Local;
use color_print::cprintln;
use sensors::Measurement;

mod ask;
mod db;
mod sensors;

fn main() {
    db::verify_drive().expect("Failed to verify drive");
    print_header();
    let team_number = ask::team_number().expect("Failed to ask user for team number");
    let mut csv_file = db::create_csv_file(&team_number).expect("Failed to create CSV file");

    loop {
        let depth = ask::depth().expect("Failed to ask for depth");
        let measurement = Measurement {
            depth,
            temp: 50.3,
            light: 0.80,
            salinity: 30.2,
            time: Local::now(),
        };
        db::write_measurement(&mut csv_file, measurement).expect("Failed to write result");
    }
}

fn print_header() {
    cprintln!("\nWelcome to <blue>BlueData</blue>!");
    println!("\nThis program will record the following information and store it in a CSV (comma separated value) file for your team. This can then be loaded into a spreadsheet program such as Microsoft Excel.");
    cprintln!("\t1. <blue>Depth</blue> (m)");
    cprintln!("\t2. <yellow>Temperature</yellow> (°C)");
    cprintln!("\t3. <magenta>Light</magenta> (% from surface)");
    cprintln!("\t4. <cyan>Salinity</cyan> (g/L)");
    println!("\t5. Time (local time)");
}
