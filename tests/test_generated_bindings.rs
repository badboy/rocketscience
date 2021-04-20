uniffi_macros::build_foreign_language_testcases!(
    "src/rocketscience.udl",
    [
        "tests/bindings/test_rocketscience.kts",
        "tests/bindings/test_rocketscience.swift",
        "tests/bindings/test_rocketscience.py",
    ]
);
