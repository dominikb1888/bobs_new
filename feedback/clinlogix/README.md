# ClinLogix

ClinLogix is a Rust-based command-line tool for health informatics and digital health workflows. It provides a single unified CLI that supports both health IT log analysis and validation of real FHIR JSON resources using an external FHIR validation service. The project includes Docker-based deployment and a GitHub Pages website for presentation and distribution.

Motivation

Healthcare IT systems generate large volumes of logs and structured clinical data. Manually inspecting these files is time-consuming, error-prone, and difficult to automate. ClinLogix was developed to support faster troubleshooting of clinical systems and to validate FHIR data before exchange or ingestion, reflecting real-world health informatics practice.

Features

- Log scanning with summarized error and warning counts  
- Error-only output for rapid troubleshooting  
- JSON output mode for automation and scripting  
- FHIR JSON validation using a FHIR $validate service  
- Clear PASS / FAIL results with detailed validation issues  
- Docker-based deployment for reproducible execution  

Command-Line Usage

Scan health IT log files:

    cargo run -- scan demo-healthit.log
    cargo run -- scan demo-healthit.log --errors-only
    cargo run -- scan demo-healthit.log --json

Validate FHIR JSON resources:

    cargo run -- validate examples/patient.json
    cargo run -- validate examples/patient-bad.json

FHIR Validation

ClinLogix validates FHIR JSON resources by calling a remote FHIR $validate endpoint. Validation responses are parsed from OperationOutcome resources and presented clearly in the terminal.

Default validation service:

    https://server.fire.ly

Docker Deployment and Execution

ClinLogix can be built and executed entirely using Docker, without requiring Rust to be installed on the host system. This enables reproducible execution across different environments.

Build the Docker image:

    docker build -t clinlogix:latest .

Run FHIR validation:

    docker run --rm -v "$PWD:/data" clinlogix:latest validate /data/examples/patient.json
    docker run --rm -v "$PWD:/data" clinlogix:latest validate /data/examples/patient-bad.json

Run log scanning:

    docker run --rm -v "$PWD:/data" clinlogix:latest scan /data/demo-healthit.log --json

Install (macOS)

After building, install ClinLogix as a system-wide CLI:

    sudo cp target/release/clinlogix /usr/local/bin/clinlogix
    clinlogix --help

Architecture Overview

[View Architecture Diagram](docs/clinlogix_architecture.png)

Deployment Summary

ClinLogix is deployed as a native Rust CLI application, as a Docker container for environment-independent execution, and as a static project website hosted on GitHub Pages.

Project Structure

    clinlogix/
    ├── src/
    │   └── main.rs
    ├── examples/
    │   ├── patient.json
    │   └── patient-bad.json
    ├── docs/
    │   ├── index.html
    │   └── Clinlogix_lean_canvas_doc.pdf
    ├── Dockerfile
    ├── .dockerignore
    ├── Cargo.toml
    └── README.md

Project Website

https://princey9.github.io/clinlogix/

Notes

Only synthetic data is used. No real patient data is included. External FHIR validation services may occasionally be unavailable.

License

MIT License

Copyright (c) 2026 Duru Princess Ifeayinwa

Permission is hereby granted to use, copy, modify, and distribute this software for academic and non-commercial purposes.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND.

Author

Duru Princess Ifeayinwa  
Health Informatics 
