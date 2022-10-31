use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub workspace: Workspace,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Workspace {
    pub members: Vec<String>,
}
