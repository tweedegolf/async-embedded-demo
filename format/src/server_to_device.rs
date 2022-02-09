use serde::{Deserialize, Serialize};

#[cfg(feature = "defmt")]
use defmt::Format;

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "defmt", derive(Format))]
pub enum ServerToDevice {
    SetLedStatus(bool),
    SayHello,
    // TODO add you own commands here for the device to handle
}