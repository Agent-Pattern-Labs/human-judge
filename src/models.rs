use std::collections::BTreeMap;
use std::fmt;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Score {
    Pass,
    Warn,
    Fail,
    Review,
}

impl Score {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pass => "pass",
            Self::Warn => "warn",
            Self::Fail => "fail",
            Self::Review => "review",
        }
    }
}

impl fmt::Display for Score {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GateStatus {
    Ship,
    Warn,
    HumanReview,
    Block,
}

impl GateStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Ship => "ship",
            Self::Warn => "warn",
            Self::HumanReview => "human_review",
            Self::Block => "block",
        }
    }
}

impl fmt::Display for GateStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Scenario {
    pub scenario_id: String,
    pub objective_id: String,
    pub name: String,
    pub risk_level: String,
    pub user_prompt: String,
    pub allowed_outcomes: Vec<String>,
    #[serde(default)]
    pub disallowed_behaviors: Vec<String>,
    pub expected_outcome: Value,
    pub judges: Vec<String>,
    #[serde(flatten)]
    pub metadata: BTreeMap<String, Value>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AgentOutput {
    #[serde(default = "default_agent_id")]
    pub agent_id: String,
    pub content: String,
    #[serde(default)]
    pub actions: Vec<String>,
    #[serde(flatten)]
    pub metadata: BTreeMap<String, Value>,
}

fn default_agent_id() -> String {
    "unknown_agent".to_string()
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct JudgeResult {
    pub judge_id: &'static str,
    pub score: Score,
    pub confidence: f32,
    pub reasons: Vec<String>,
    pub evidence: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_fix: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct GateDecision {
    pub decision: GateStatus,
    pub reason: String,
    pub blocking_judges: Vec<&'static str>,
    pub review_judges: Vec<&'static str>,
    pub warning_judges: Vec<&'static str>,
}
