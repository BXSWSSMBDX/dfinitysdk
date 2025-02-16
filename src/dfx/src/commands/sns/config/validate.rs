use crate::{DfxResult, Environment};

use crate::lib::sns;
use crate::lib::sns::validate_config::validate_config;
use clap::Parser;

/// Validates an sns configuration
#[derive(Parser)]
pub struct ValidateOpts {}

pub fn exec(env: &dyn Environment, _opts: ValidateOpts) -> DfxResult {
    let config = env.get_config_or_anyhow()?;
    let path = config.get_project_root().join(sns::CONFIG_FILE_NAME);

    validate_config(&path)
}
