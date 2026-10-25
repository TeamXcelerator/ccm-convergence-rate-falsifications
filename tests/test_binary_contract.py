"""The same version label must not admit a stale or feature-incomplete binary."""
import importlib.util
from pathlib import Path
import re
import unittest

ROOT = Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location("check_binary", ROOT / "scripts/check_binary.py")
check = importlib.util.module_from_spec(spec)
spec.loader.exec_module(check)

class BinaryContract(unittest.TestCase):
    def setUp(self):
        revision = re.search(r'rev\s*=\s*"([0-9a-f]{40})"', (ROOT / "Cargo.toml").read_text(encoding="utf-8")).group(1)
        self.info = dict(schema_version=1, toolkit_revision=revision,
                         implementation_digest=check.expected_digest(ROOT), complete_positive_roots=True)

    def test_matching_build_is_accepted(self):
        check.validate(ROOT, self.info)

    def test_stale_source_is_rejected(self):
        self.info["implementation_digest"] = "0" * 64
        with self.assertRaisesRegex(ValueError, "sources and Cargo.lock"):
            check.validate(ROOT, self.info)

    def test_wrong_toolkit_is_rejected(self):
        self.info["toolkit_revision"] = "0" * 40
        with self.assertRaisesRegex(ValueError, "toolkit revision"):
            check.validate(ROOT, self.info)

    def test_missing_complete_range_feature_is_rejected(self):
        self.info["complete_positive_roots"] = False
        with self.assertRaisesRegex(ValueError, "complete-range"):
            check.validate(ROOT, self.info)

if __name__ == "__main__":
    unittest.main()
