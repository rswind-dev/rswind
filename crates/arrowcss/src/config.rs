use serde::Deserialize;

use crate::theme::Theme;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArrowConfig {
  pub dark_mode: String,
  pub theme: Theme,
}
