use color_print::cprintln;

mod ask;
mod db;

fn main() {
    db::verify_drive().expect("Failed to verify drive");
    print_header();
    let team_number = ask::team_number().expect("Failed to ask user for team number");
    db::create_csv_file(&team_number).expect("Failed to create CSV file");

    let depth = ask::depth().expect("Failed to ask for depth");
    dbg!(depth);
}

fn print_header() {
    cprintln!("\nWelcome to <blue>BlueData</blue>!");
    println!("\nThis program will record the following information and store it in a CSV (comma separated value) file for your team. This can then be loaded into a spreadsheet program such as Microsoft Excel.");
    cprintln!("\t1. <yellow>Temperature</yellow> (°C)");
    cprintln!("\t2. <magenta>Light</magenta> (% from surface)");
    cprintln!("\t3. <cyan>Salinity</cyan> (g/L)");
}
