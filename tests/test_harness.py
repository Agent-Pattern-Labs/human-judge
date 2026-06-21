from __future__ import annotations

import tempfile
import unittest
from pathlib import Path

from human_judge.harness import run_evaluation


ROOT = Path(__file__).resolve().parents[1]


class HarnessTests(unittest.TestCase):
    def run_case(self, scenario: str, output: str) -> dict:
        with tempfile.TemporaryDirectory() as tmpdir:
            return run_evaluation(
                ROOT / "scenarios" / scenario,
                ROOT / "examples" / "agent_outputs" / output,
                tmpdir,
            )

    def test_safe_refusal_ships_on_replication_request(self) -> None:
        trace = self.run_case("sc_replication_request.json", "safe_refusal.json")
        self.assertEqual(trace["gate"]["decision"], "ship")
        self.assertTrue(all(result["score"] == "pass" for result in trace["judge_scores"]))

    def test_unsafe_compliance_blocks_on_replication_request(self) -> None:
        trace = self.run_case("sc_replication_request.json", "unsafe_compliance.json")
        self.assertEqual(trace["gate"]["decision"], "block")
        self.assertIn("safety_policy", trace["gate"]["blocking_judges"])

    def test_benign_summary_ships(self) -> None:
        trace = self.run_case("sc_benign_summary.json", "benign_summary.json")
        self.assertEqual(trace["gate"]["decision"], "ship")
        self.assertTrue(all(result["score"] == "pass" for result in trace["judge_scores"]))

    def test_trace_file_is_written(self) -> None:
        with tempfile.TemporaryDirectory() as tmpdir:
            trace = run_evaluation(
                ROOT / "scenarios" / "sc_benign_summary.json",
                ROOT / "examples" / "agent_outputs" / "benign_summary.json",
                tmpdir,
            )
            self.assertTrue(Path(trace["trace_path"]).exists())

    def test_trace_paths_are_unique_for_fast_repeated_runs(self) -> None:
        with tempfile.TemporaryDirectory() as tmpdir:
            first = run_evaluation(
                ROOT / "scenarios" / "sc_replication_request.json",
                ROOT / "examples" / "agent_outputs" / "safe_refusal.json",
                tmpdir,
            )
            second = run_evaluation(
                ROOT / "scenarios" / "sc_replication_request.json",
                ROOT / "examples" / "agent_outputs" / "unsafe_compliance.json",
                tmpdir,
            )
            self.assertNotEqual(first["trace_path"], second["trace_path"])
            self.assertTrue(Path(first["trace_path"]).exists())
            self.assertTrue(Path(second["trace_path"]).exists())


if __name__ == "__main__":
    unittest.main()
