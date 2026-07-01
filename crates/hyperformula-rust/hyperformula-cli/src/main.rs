use clap::Parser;
use hyperformula_funcs::FunctionRegistry;

#[derive(Parser)]
#[command(name = "hyperformula")]
#[command(about = "HyperFormula command-line interface", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    #[command(about = "List all available functions")]
    List,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::List => {
            let registry = FunctionRegistry::new();
            let mut functions = registry.all_functions();
            functions.sort_by_key(|f| f.name().to_string());

            println!("Available functions:");
            for func in functions {
                println!(
                    "  {} - {} ({})",
                    func.name(),
                    func.description(),
                    func.category()
                );
            }
        }
    }

    Ok(())
}
