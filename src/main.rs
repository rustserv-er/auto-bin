use clap::{Arg, Command};
use std::{fs, path::Path};
use toml_edit::{DocumentMut, Item, Value, Table};
use anyhow::{Context, Result};

fn main() {
    let matches = Command::new("auto-bin")
        .version("0.1.0")
        .author("Sarvesh <rajeshbaranwal9044@gmail.com>")
        .about("Boosts productivity by automating bin setup in Rust projects 🚀")
        .arg(
            Arg::new("init")
                .short('i')
                .long("init")
                .help("Initialize auto-bin in current project")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("status")
                .short('s')
                .long("status")
                .help("Check current auto-bin configuration")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    if matches.contains_id("init") {
        println!("🚀 Initializing auto-bin for your Rust project...");
        run_init();
    } else if matches.contains_id("status") {
        println!("📊 Checking auto-bin status...");
        run_status();
    } else {
        println!("ℹ️ Use --help to see available commands.");
    }
}

fn run_init() {
    if let Err(e) = do_run_init() {
        eprintln!("❌ Error: {}", e);
    }
}

fn do_run_init() -> Result<()> {
    let cargo_toml_path = Path::new("Cargo.toml");

    if !cargo_toml_path.exists() {
        anyhow::bail!("Cargo.toml not found. Run inside a Rust project.");
    }

    let src_bin = Path::new("src/bin");
    if !src_bin.exists() {
        println!("⚠️ No src/bin directory found. Nothing to do.");
        return Ok(());
    }

    let mut doc: DocumentMut = fs::read_to_string(cargo_toml_path)
        .context("Failed to read Cargo.toml")?
        .parse::<DocumentMut>()
        .context("Failed to parse Cargo.toml")?;

    // Ensure [[bin]] exists
    if doc.get("bin").is_none() {
        doc.insert("bin", Item::ArrayOfTables(Default::default()));
    }

    // Get mutable reference to [[bin]]
    let bins = doc.get_mut("bin")
        .and_then(Item::as_array_of_tables_mut)
        .context("Failed to get [[bin]] as array-of-tables")?;

    let mut added = 0usize;
    for entry in fs::read_dir(src_bin).context("Failed to read src/bin")? {
        let path = entry?.path();
        if path.extension().and_then(|s| s.to_str()) != Some("rs") {
            continue;
        }

        let name = path.file_stem().unwrap().to_string_lossy().to_string();
        let rel_path = format!("src/bin/{}.rs", name);

        let exists = bins.iter().any(|t| {
            t.get("name")
                .and_then(Item::as_value)
                .and_then(Value::as_str)
                == Some(name.as_str())
                || t.get("path")
                    .and_then(Item::as_value)
                    .and_then(Value::as_str)
                    == Some(rel_path.as_str())
        });

        if !exists {
            let mut t = Table::new();
            t["name"] = Item::Value(Value::from(name.clone()));
            t["path"] = Item::Value(Value::from(rel_path));
            bins.push(t);
            added += 1;
        }
    }

    if added > 0 {
        fs::write(cargo_toml_path, doc.to_string()).context("Failed to write Cargo.toml")?;
        println!("✅ Added {} bin target(s) to Cargo.toml", added);
    } else {
        println!("✅ Nothing to add. Cargo.toml is already in sync.");
    }

    Ok(())
}

fn run_status() {
    if let Err(e) = do_run_status() {
        eprintln!("❌ Error: {}", e);
    }
}

fn do_run_status() -> Result<()> {
    let cargo_toml_path = Path::new("Cargo.toml");
    if !cargo_toml_path.exists() {
        anyhow::bail!("Cargo.toml not found. Are you inside a Rust project?");
    }

    let content = fs::read_to_string(cargo_toml_path)?;
    let doc: DocumentMut = content.parse::<DocumentMut>()?;

    let mut declared: Vec<String> = Vec::new();

    if let Some(bins) = doc.get("bin").and_then(Item::as_array_of_tables) {
        for t in bins.iter() {
            if let Some(name) = t.get("name")
                .and_then(Item::as_value)
                .and_then(Value::as_str)
            {
                declared.push(name.to_string());
            }
        }
    }

    let src_bin = Path::new("src/bin");
    let mut detected: Vec<String> = Vec::new();

    if src_bin.exists() {
        for entry in fs::read_dir(src_bin)? {
            let path = entry?.path();
            if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                detected.push(path.file_stem().unwrap().to_string_lossy().to_string());
            }
        }
    }

    println!("📦 Declared in Cargo.toml: {:?}", declared);
    println!("🔍 Detected in src/bin:   {:?}", detected);

    let missing: Vec<_> = detected
        .iter()
        .filter(|n| !declared.contains(n))
        .cloned()
        .collect();

    if missing.is_empty() {
        println!("✅ In sync");
    } else {
        println!("⚠️ Missing in Cargo.toml: {:?}", missing);
    }

    Ok(())
}
