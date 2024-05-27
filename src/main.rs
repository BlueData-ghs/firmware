use color_print::cprintln;

mod ask;
mod data;

fn main() {
    print_header();
    let depth = ask::depth().expect("Failed to ask for depth");
}

fn print_header() {
    cprintln!("\n<blue>BlueData</blue>");
    println!("\nYou will be asked at what depth you're trying to measure your data over and over again. If you are completely done measuring simply input \"done\" as your depth.");
}
