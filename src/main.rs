use clap::{Arg, Command};

fn main() {
    let matches = Command::new("auto-bin")
        .version("0.1.0")
        .author("Your Name <youremail@example.com>")
        .about("Boosts productivity by automating bin setup in Rust projects 🚀")
        .arg(
            Arg::new("init")
                .short('i')
                .long("init")
                .help("Initialize auto-bin in current project")
        )
        .arg(
            Arg::new("status")
                .short('s')
                .long("status")
                .help("Check current auto-bin configuration")
        )
        .get_matches();

    if matches.contains_id("init") {
        println!("🚀 Initializing auto-bin for your Rust project...");
    } else if matches.contains_id("status") {
        println!("📦 Auto-bin status: [Not implemented yet]");
    } else {
        println!("ℹ️ Use --help to see available commands.");
    }
}


fn run_init(){
    let cargo_toml=Path:: new("cargo.toml");
    if !cargo_toml.exists(){
        eprintln!("❌ Error: Cargo.toml not found. Run inside a Rust project.");
        return ;
    }

    let bin_dir=Path:: new("src/bin");
    if !bin_dir.exists(){
        eprintln!("⚠️ No src/bin directory found. Nothing to do.");
    }

    for entry in fs::read_dir(bin_dir).unwrap(){
        let entry=entry.unwrap();
        let path=entry.path();

        if path.exetension().and_then(|s| s.to_str())==Some("rs"){
            let file_stem=path.file_stem().unwrap()
.to_str().unwrap();
println!("Found binary:{}",file_stem);
}
    }
}