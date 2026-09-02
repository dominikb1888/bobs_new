use serde::Serialize;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Serialize)]
pub struct ScanReport {
    pub file: String,
    pub total_lines: u64,
    pub errors: u64,
    pub warnings: u64,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub error_lines: Vec<String>,
}

pub fn scan_logfile(logfile: &str, errors_only: bool) -> io::Result<ScanReport> {
    let file = File::open(logfile)?;
    let reader = BufReader::new(file);

    let mut total_lines: u64 = 0;
    let mut error_count: u64 = 0;
    let mut warning_count: u64 = 0;
    let mut error_lines: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        total_lines += 1;

        let lower = line.to_lowercase();
        if lower.contains("error") {
            error_count += 1;
            if errors_only {
                error_lines.push(line);
            }
        } else if lower.contains("warning") {
            warning_count += 1;
        }
    }

    Ok(ScanReport {
        file: logfile.to_string(),
        total_lines,
        errors: error_count,
        warnings: warning_count,
        error_lines,
    })
}
