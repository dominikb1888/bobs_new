use clap::{Parser, Subcommand};

mod model;
mod scan;
mod validate;

/// ClinLogix: Health IT utility CLI (log scan + FHIR validation)
#[derive(Parser, Debug)]
#[command(name = "clinlogix", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Scan a log file and summarize errors/warnings
    Scan {
        /// Path to the log file
        logfile: String,

        /// Print only error lines
        #[arg(long)]
        errors_only: bool,

        /// Output summary as JSON
        #[arg(long)]
        json: bool,
    },

    /// Validate a FHIR JSON resource using a FHIR server ($validate)
    Validate {
        /// Path to a FHIR JSON file (e.g., examples/patient.json)
        fhir_file: String,

        /// FHIR base URL (defaults to public HAPI R4 server)
        #[arg(long, default_value = "https://server.fire.ly")]
        base_url: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan {
            logfile,
            errors_only,
            json,
        } => {
            let report = scan::scan_logfile(&logfile, errors_only)?;
            if errors_only {
                for line in report.error_lines {
                    println!("{}", line);
                }
            }
            else if json {
                println!("{}", serde_json::to_string_pretty(&report)?);
            }
            else {
                println!("ClinLogix Report");
                println!("----------------");
                println!("File: {}", report.file);
                println!("Total lines: {}", report.total_lines);
                println!("Errors: {}", report.errors);
                println!("Warnings: {}", report.warnings);
            }
        }

        Commands::Validate { fhir_file, base_url } => {
            if let Err(e) = validate::run_validate(&fhir_file, &base_url).await {
                eprintln!("Validation failed: {}", e);
                std::process::exit(1);
            }
        }
    }

    Ok(())
}
