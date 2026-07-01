use anyhow::{Context, Result};
use clap::Parser;
use std::path::PathBuf;

mod generator;
mod scraper;
mod spec;

#[derive(Parser)]
#[command(name = "hyperformula-codegen")]
#[command(about = "Generate Rust function stubs from HyperFormula documentation", long_about = None)]
struct Cli {
    #[arg(
        long,
        default_value = "https://hyperformula.handsontable.com/guide/built-in-functions.html"
    )]
    source_url: String,

    #[arg(long, default_value = "../hyperformula-funcs/src")]
    out_dir: PathBuf,

    #[arg(long, default_value = "../hyperformula-tests/src")]
    tests_out: PathBuf,

    #[arg(long, default_value = "./out")]
    spec_out: PathBuf,

    #[arg(long)]
    force: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    println!("HyperFormula Code Generator");
    println!("==========================");
    println!();

    println!("Fetching function spec from: {}", cli.source_url);
    let html =
        scraper::fetch_html(&cli.source_url).context("Failed to fetch HTML from source URL")?;

    println!("Parsing function specifications...");
    let functions =
        scraper::parse_functions(&html).context("Failed to parse functions from HTML")?;

    println!("Found {} functions", functions.len());

    std::fs::create_dir_all(&cli.spec_out).context("Failed to create spec output directory")?;

    let spec_path = cli.spec_out.join("spec.json");
    println!("Writing spec to: {}", spec_path.display());
    generator::write_spec_json(&functions, &spec_path).context("Failed to write spec.json")?;

    println!("Generating Rust function stubs...");
    let generated_rs_path = cli.out_dir.join("generated.rs");
    generator::generate_functions(&functions, &generated_rs_path)
        .context("Failed to generate function stubs")?;
    println!("  Generated: {}", generated_rs_path.display());

    println!("Generating test skeletons...");
    let tests_path = cli.tests_out.join("generated_tests.rs");
    generator::generate_tests(&functions, &tests_path)
        .context("Failed to generate test skeletons")?;
    println!("  Generated: {}", tests_path.display());

    println!();
    println!("Code generation complete!");
    println!();
    println!("Next steps:");
    println!("  1. Review generated spec: {}", spec_path.display());
    println!("  2. Build workspace: cargo build --workspace");
    println!("  3. Run tests: cargo test --workspace");
    println!("  4. Implement functions by replacing stubs in generated.rs");

    Ok(())
}
