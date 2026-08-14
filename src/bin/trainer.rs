//! CLI entry point for the trainer.

use clap::Parser;
use lift_ops_trainer::parser;
use lift_ops_trainer::enumerator;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "trainer", about = "LIFT OPS · Trainer CLI")]
struct Args {
    /// Path to a content YAML file
    #[arg(short, long)]
    file: Option<PathBuf>,

    /// List critical checklist items
    #[arg(long)]
    critical: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    println!("LIFT OPS · Trainer");
    println!("==================");
    println!("Study aid only. Not a certification.");
    println!();

    if let Some(path) = args.file {
        let doc = parser::parse_yaml_file(&path)?;
        if args.critical {
            let items = enumerator::critical_items(&doc);
            println!("Critical items ({}):", items.len());
            for (i, it) in items.iter().enumerate() {
                println!("  {}. {}", i + 1, it.text);
            }
        } else {
            println!("Loaded document from {}", path.display());
            println!("{:#?}", doc);
        }
    } else {
        println!("No file given. Try:");
        println!("  cargo run --bin trainer -- --file content/checklists/boom.yaml --critical");
        println!();
        println!("Phone / Termux: open web/index.html");
    }

    Ok(())
}
