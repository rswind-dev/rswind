use serde::{Deserialize, Serialize};

use crate::theme::Theme;


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArrowConfig {
  pub dark_mode: String,
  pub theme: Theme,
}