use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use chrono::{SecondsFormat, Utc};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

use crate::gate::decide_gate;
use crate::judges::run_default_judges;
use crate::models::{AgentOutput, GateStatus, Scenario};

pub type HarnessResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

pub fn load_scenario(path: impl AsRef<Path>) -> HarnessResult<Scenario> {
    let data = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&data)?)
}

pub fn load_agent_output(path: impl AsRef<Path>) -> HarnessResult<AgentOutput> {
    let path = path.as_ref();
    let data = fs::read_to_string(path)?;
    if path
        .extension()
        .is_some_and(|extension| extension == "json")
    {
        return Ok(serde_json::from_str(&data)?);
    }

    let agent_id = path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("unknown_agent")
        .to_string();
    Ok(AgentOutput {
        agent_id,
        content: data,
        actions: vec![],
        metadata: Default::default(),
    })
}

pub fn write_json(path: impl AsRef<Path>, data: &Value) -> HarnessResult<()> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let rendered = serde_json::to_string_pretty(data)?;
    fs::write(path, format!("{rendered}\n"))?;
    Ok(())
}

pub fn hash_text(text: &str) -> String {
    let digest = Sha256::digest(text.as_bytes());
    format!("{digest:x}").chars().take(16).collect()
}

pub fn safe_trace_part(value: &str) -> String {
    let mut output = String::new();
    let mut previous_was_separator = false;
    for character in value.chars() {
        let safe = character.is_ascii_alphanumeric() || matches!(character, '.' | '_' | '-');
        if safe {
            output.push(character);
            previous_was_separator = false;
        } else if !previous_was_separator {
            output.push('_');
            previous_was_separator = true;
        }
    }
    let trimmed = output.trim_matches('_');
    if trimmed.is_empty() {
        "unknown".to_string()
    } else {
        trimmed.to_string()
    }
}

pub fn run_evaluation(
    scenario_path: impl AsRef<Path>,
    agent_output_path: impl AsRef<Path>,
    trace_dir: impl AsRef<Path>,
) -> HarnessResult<Value> {
    let scenario_path = scenario_path.as_ref();
    let agent_output_path = agent_output_path.as_ref();
    let scenario = load_scenario(scenario_path)?;
    let output = load_agent_output(agent_output_path)?;
    let started_at = Utc::now();
    let judge_results = run_default_judges(&scenario, &output);
    let gate = decide_gate(&scenario, &judge_results);
    let ended_at = Utc::now();
    let trace_id = format!(
        "tr_{}_{}_{}",
        safe_trace_part(&scenario.scenario_id),
        safe_trace_part(&output.agent_id),
        ended_at.format("%Y%m%dT%H%M%S%6fZ")
    );
    let trace_path: PathBuf = trace_dir.as_ref().join(format!("{trace_id}.json"));

    let mut trace = json!({
        "trace_id": trace_id,
        "scenario_id": scenario.scenario_id,
        "objective_id": scenario.objective_id,
        "environment": "local",
        "started_at": started_at.to_rfc3339_opts(SecondsFormat::Micros, true),
        "ended_at": ended_at.to_rfc3339_opts(SecondsFormat::Micros, true),
        "actor": {
            "type": "agent_output_file",
            "agent_id": output.agent_id,
        },
        "world": {
            "risk_level": scenario.risk_level,
            "scenario_file": scenario_path.to_string_lossy(),
            "agent_output_file": agent_output_path.to_string_lossy(),
        },
        "inputs": {
            "prompt_hash": hash_text(&scenario.user_prompt),
            "output_hash": hash_text(&output.content),
            "proposed_actions": output.actions,
        },
        "outcome": {
            "task_completed": matches!(gate.decision, GateStatus::Ship | GateStatus::Warn),
            "errors": 0,
            "support_needed": matches!(gate.decision, GateStatus::HumanReview),
        },
        "judge_scores": judge_results,
        "gate": gate,
    });

    write_json(&trace_path, &trace)?;
    if let Some(object) = trace.as_object_mut() {
        object.insert(
            "trace_path".to_string(),
            Value::String(trace_path.to_string_lossy().to_string()),
        );
    }
    Ok(trace)
}
