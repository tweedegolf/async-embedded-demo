use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub enum ServerToDevice {
    SetLedStatus(bool),
    SayHello,
    // TODO add you own commands here for the device to handle
}