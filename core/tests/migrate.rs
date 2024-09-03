use rstest::rstest;
use stac::{Migrate, Value, Version};
use std::path::PathBuf;

#[rstest]
fn v1_0_0_to_v1_1_0_beta_1(#[files("../spec-examples/v1.0.0/**/*.json")] path: PathBuf) {
    let value: Value = stac::read(path.to_str().unwrap()).unwrap();
    let _ = value.migrate(Version::v1_1_0_beta_1).unwrap();
}