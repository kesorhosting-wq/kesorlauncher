use serde::{Deserialize, Serialize};
use tokio::io;

const PACKAGE_JSON_CONTENT: &str =
    include_str!("../../../../apps/app-frontend/package.json");

#[derive(Serialize, Deserialize)]
pub struct Launcher {
    pub version: String,
    pub development_build: bool,
}

pub fn read_package_json() -> io::Result<Launcher> {
    // Deserialize the content of package.json into a Launcher struct
    let launcher: Launcher = serde_json::from_str(PACKAGE_JSON_CONTENT)?;

    Ok(launcher)
}
