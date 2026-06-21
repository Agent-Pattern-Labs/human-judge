use std::fs;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::harness::{HarnessResult, run_evaluation};

#[derive(Debug, Parser)]
#[command(
    name = "human-judge",
    about = "Run safety judge scenarios against agent outputs."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Run one scenario evaluation.
    Run {
        /// Path to scenario JSON.
        #[arg(long)]
        scenario: PathBuf,
        /// Path to an agent output JSON or text file.
        #[arg(long = "agent-output")]
        agent_output: PathBuf,
        /// Directory where trace JSON should be written.
        #[arg(long = "trace-dir", default_value = "qa-loop/traces/regression")]
        trace_dir: PathBuf,
        /// Print the full trace JSON instead of a concise summary.
        #[arg(long)]
        json: bool,
    },
    /// List scenario files.
    List {
        /// Directory containing scenario JSON files.
        #[arg(long = "scenario-dir", default_value = "scenarios")]
        scenario_dir: PathBuf,
    },
}

pub fn run() -> HarnessResult<u8> {
    let cli = Cli::parse();
    match cli.command {
        Commands::List { scenario_dir } => {
            println!("{}", list_scenarios(scenario_dir)?);
            Ok(0)
        }
        Commands::Run {
            scenario,
            agent_output,
            trace_dir,
            json,
        } => {
            let trace = run_evaluation(scenario, agent_output, trace_dir)?;
            if json {
                println!("{}", serde_json::to_string_pretty(&trace)?);
            } else {
                println!("{}", summarize(&trace));
            }
            Ok(exit_code_for_gate(
                trace["gate"]["decision"].as_str().unwrap_or("block"),
            ))
        }
    }
}

fn list_scenarios(scenario_dir: PathBuf) -> HarnessResult<String> {
    let mut paths: Vec<String> = fs::read_dir(&scenario_dir)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension == "json")
        })
        .map(|path| path.to_string_lossy().to_string())
        .collect();
    paths.sort();

    if paths.is_empty() {
        return Ok(format!("No scenarios found in {}", scenario_dir.display()));
    }
    Ok(paths.join("\n"))
}

fn summarize(trace: &serde_json::Value) -> String {
    let gate = &trace["gate"];
    let mut lines = vec![
        format!("Gate: {}", gate["decision"].as_str().unwrap_or("unknown")),
        format!("Reason: {}", gate["reason"].as_str().unwrap_or("unknown")),
        format!(
            "Trace: {}",
            trace["trace_path"].as_str().unwrap_or("unknown")
        ),
        "Scores:".to_string(),
    ];

    if let Some(scores) = trace["judge_scores"].as_array() {
        for score in scores {
            let reasons = score["reasons"]
                .as_array()
                .map(|values| {
                    values
                        .iter()
                        .filter_map(|value| value.as_str())
                        .collect::<Vec<_>>()
                        .join("; ")
                })
                .unwrap_or_default();
            lines.push(format!(
                "- {}: {} (confidence={:.2}) - {}",
                score["judge_id"].as_str().unwrap_or("unknown"),
                score["score"].as_str().unwrap_or("unknown"),
                score["confidence"].as_f64().unwrap_or_default(),
                reasons
            ));
        }
    }

    lines.join("\n")
}

fn exit_code_for_gate(decision: &str) -> u8 {
    match decision {
        "ship" | "warn" => 0,
        "human_review" => 3,
        "block" => 2,
        _ => 1,
    }
}
