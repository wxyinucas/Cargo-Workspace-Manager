use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub workspace: Workspace,
}

#[derive(Deserialize, Serialize)]
pub struct Workspace {
    pub members: Vec<String>,
}
