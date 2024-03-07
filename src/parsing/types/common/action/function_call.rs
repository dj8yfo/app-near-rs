use near_gas::NearGas;
use near_token::NearToken;

use crate::utils::types::capped_string::CappedString;
use crate::utils::types::hex_display::HexDisplay;
use borsh::io::{Read, Result};
use borsh::BorshDeserialize;

#[cfg(target_os = "nanos")]
pub type ArgsStringRepresentation = CappedString<200>;
#[cfg(target_os = "nanos")]
pub type ArgsBinaryRepresentation = HexDisplay<200>;
#[cfg(target_os = "nanosplus")]
pub type ArgsStringRepresentation = CappedString<1000>;
#[cfg(target_os = "nanosplus")]
pub type ArgsBinaryRepresentation = HexDisplay<1000>;
#[cfg(target_os = "nanox")]
pub type ArgsStringRepresentation = CappedString<1000>;
#[cfg(target_os = "nanox")]
pub type ArgsBinaryRepresentation = HexDisplay<1000>;

pub struct FunctionCallCommon {
    pub method_name: CappedString<50>,
    pub gas: NearGas,
    pub deposit: NearToken,
}

impl FunctionCallCommon {
    pub fn deserialize_with_method_name<R: Read>(
        reader: &mut R,
        method_name: CappedString<50>,
    ) -> Result<Self> {
        let gas: NearGas = BorshDeserialize::deserialize_reader(reader)?;
        let deposit: NearToken = BorshDeserialize::deserialize_reader(reader)?;

        let r = Self {
            method_name,
            gas,
            deposit,
        };
        Ok(r)
    }
}
