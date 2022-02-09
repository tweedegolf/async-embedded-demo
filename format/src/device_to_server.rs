use serde::{Deserialize, Serialize};

#[cfg(feature = "defmt")]
use defmt::Format;

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "defmt", derive(Format))]
pub enum DeviceToServer {
    Hello,
    // TODO add you own fields here for the CLI to handle
}