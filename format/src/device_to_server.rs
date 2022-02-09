use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub enum DeviceToServer {
    Hello,
    // TODO add you own fields here for the CLI to handle
}