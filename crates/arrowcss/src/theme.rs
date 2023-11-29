use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Theme {
    pub colors: HashMap<String, String>,
    pub spacing: HashMap<String, String>,
}
