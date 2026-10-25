"""Failed acquisition journals must retain their cause without measurements."""
import contextlib
import importlib.util
import io
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location("summarize_runs", ROOT / "scripts/summarize_runs.py")
summarize = importlib.util.module_from_spec(spec)
spec.loader.exec_module(summarize)


class SummarizeRuns(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        self.run = self.root / "run-1"
        self.run.mkdir()
        self.error = "primary acquisition: exact polynomial budget exceeded"
        self.summary = dict(schema_version=1, run_status="INCOMPLETE", capture_status="NOT_RUN",
                            measurements_saved=False, checks=[dict(
                                category="validation", name="required computation",
                                status="INCOMPLETE", reason=self.error)])
        self.write("summary", self.summary)
        self.write("request", {"resolved_C_N_P_root_count": [13, 120, 1000, 8]})
        self.write("build", {"toolkit_revision": "test-fixture"})
        self.write("failure", dict(error=self.error, primary_saved=False, measurements_saved=False))

    def write(self, name, value):
        (self.run / (name + ".json")).write_text(json.dumps(value), encoding="utf-8")

    def assess(self):
        output = io.StringIO()
        with contextlib.redirect_stdout(output):
            report = summarize.assess(self.root)
        return report, output.getvalue()

    def test_primary_failure_preserves_available_evidence_without_measurements(self):
        report, output = self.assess()
        self.assertFalse(report["complete"])
        record = report["runs"][0]
        self.assertEqual(record["failure"]["error"], self.error)
        self.assertEqual(record["summary"], self.summary)
        self.assertEqual(record["request"]["resolved_C_N_P_root_count"], [13, 120, 1000, 8])
        self.assertEqual(record["build"]["toolkit_revision"], "test-fixture")
        self.assertIsNone(record["measurements"])
        self.assertIn(self.error, output)
        self.assertIn("Applicable capture completeness: NOT_RUN", output)
        self.assertIn("measurements were not produced", output)
        self.assertNotIn("No such file", output)

    def test_old_failed_summary_with_capture_pass_remains_incomplete(self):
        self.summary["capture_status"] = "PASS"
        self.write("summary", self.summary)
        report, output = self.assess()
        self.assertFalse(report["complete"])
        self.assertIn(self.error, output)
        self.assertEqual(report["runs"][0]["summary"]["capture_status"], "PASS")

    def test_missing_failure_artifact_still_preserves_error_from_checks(self):
        (self.run / "failure.json").unlink()
        report, output = self.assess()
        self.assertFalse(report["complete"])
        self.assertIn(self.error, output)
        self.assertEqual(report["runs"][0]["summary"]["checks"][0]["reason"], self.error)

    def test_missing_summary_does_not_hide_primary_failure_or_other_artifacts(self):
        (self.run / "summary.json").unlink()
        report, output = self.assess()
        self.assertFalse(report["complete"])
        self.assertIn(self.error, output)
        self.assertIn("build", report["runs"][0])
        self.assertEqual(report["runs"][0]["failure"]["error"], self.error)

    def make_success(self):
        (self.run / "failure.json").unlink()
        self.summary.update(run_status="PASS", capture_status="PASS", measurements_saved=True,
                            checks=[dict(category="hypothesis", name="finite comparison",
                                         status="FAIL", reason="negative finite result")])
        self.write("summary", self.summary)
        self.write("measurements", {"observable": "test", "values": {"result": 1},
                                    "checks": self.summary["checks"]})

    def test_successful_negative_hypothesis_is_still_a_complete_run(self):
        self.make_success()
        report, _ = self.assess()
        self.assertTrue(report["complete"])
        self.assertEqual(report["check_counts"]["FAIL"], 1)
        self.assertEqual(report["runs"][0]["measurements"]["values"], {"result": 1})

    def test_claimed_measurements_missing_from_disk_are_incomplete(self):
        self.make_success()
        (self.run / "measurements.json").unlink()
        report, output = self.assess()
        self.assertFalse(report["complete"])
        self.assertIn("missing despite measurements_saved=true", output)
        self.assertIn("summary", report["runs"][0])

    def test_malformed_measurements_do_not_discard_failure(self):
        (self.run / "measurements.json").write_text("{", encoding="utf-8")
        report, output = self.assess()
        self.assertFalse(report["complete"])
        self.assertIn(self.error, output)
        self.assertEqual(report["runs"][0]["failure"]["error"], self.error)

    def test_nonobject_measurements_cannot_pass(self):
        self.make_success()
        for value in (None, [], "not measurements", 1):
            with self.subTest(value=value):
                self.write("measurements", value)
                report, output = self.assess()
                self.assertFalse(report["complete"])
                self.assertIn("measurements.json: expected a JSON object", output)

    def test_measurements_require_the_basic_measurement_fields(self):
        self.make_success()
        for value in ({}, {"observable": "test", "checks": []},
                      {"observable": None, "checks": [], "values": {}},
                      {"observable": "test", "values": {}, "checks": None},
                      {"observable": "test", "values": {}, "checks": {}},
                      {"observable": "test", "values": {}, "checks": [None]}):
            with self.subTest(value=value):
                self.write("measurements", value)
                report, output = self.assess()
                self.assertFalse(report["complete"])
                self.assertIn("measurements.json:", output)

    def test_metadata_artifacts_must_be_objects(self):
        self.make_success()
        for name in ("request", "build", "failure"):
            for value in (None, []):
                with self.subTest(name=name, value=value):
                    self.write(name, value)
                    report, output = self.assess()
                    self.assertFalse(report["complete"])
                    self.assertIn(name + ".json: expected a JSON object", output)
                    self.assertIn("measurements", report["runs"][0])
            if name == "failure":
                (self.run / "failure.json").unlink()
            else:
                self.write(name, {})

    def test_malformed_summary_check_container_cannot_pass(self):
        self.make_success()
        self.summary["checks"] = {}
        self.write("summary", self.summary)
        report, output = self.assess()
        self.assertFalse(report["complete"])
        self.assertIn("summary.json: checks must be an array", output)

    def test_cli_retains_failure_in_aggregate_and_returns_nonzero(self):
        aggregate = self.root / "aggregate.json"
        result = subprocess.run([sys.executable, str(ROOT / "scripts/summarize_runs.py"),
                                 str(self.root), "--output", str(aggregate)],
                                capture_output=True, text=True, check=False)
        self.assertEqual(result.returncode, 1, result.stderr)
        self.assertIn(self.error, result.stdout)
        self.assertEqual(json.loads(aggregate.read_text(encoding="utf-8"))["runs"][0]
                         ["failure"]["error"], self.error)


if __name__ == "__main__":
    unittest.main()
