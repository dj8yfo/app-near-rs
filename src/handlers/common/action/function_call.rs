use crate::io::ErrorKind;
use crate::parsing::types::FunctionCallCommon;
use crate::sign_ui;
use crate::utils::types::capped_string::CappedString;
use crate::{
    parsing::{borsh::BorshDeserialize, HashingStream, SingleTxStream},
    AppSW,
};
#[cfg(feature = "speculos")]
use ledger_device_sdk::testing;

use super::ActionParams;

pub fn handle(
    stream: &mut HashingStream<SingleTxStream<'_>>,
    params: ActionParams,
) -> Result<(), AppSW> {
    let mut method_name: CappedString<50> = CappedString::new();

    method_name
        .deserialize_reader_in_place(stream)
        .map_err(|_err| AppSW::TxParsingFail)?;

    let args_bytes_count: u32 =
        u32::deserialize_reader(stream).map_err(|_err| AppSW::TxParsingFail)?;

    match stream
        .reader
        .peek_u8()
        .map_err(|_err| AppSW::TxParsingFail)?
    {
        // '{' char
        Some(123) => {
            let mut args_str: CappedString<500> = CappedString::new();
            match args_str.deserialize_with_bytes_count(stream, args_bytes_count) {
                Err(err) if err.kind() == ErrorKind::InvalidData => {
                    return Err(AppSW::TxParsingFail);
                }
                Ok(_) => {
                    let func_call_common =
                        FunctionCallCommon::deserialize_with_method_name(stream, method_name)
                            .map_err(|_err| AppSW::TxParsingFail)?;
                    #[cfg(feature = "speculos")]
                    debug_print(&args_str, &func_call_common);
                    if !sign_ui::action::ui_display_function_call_str(
                        &func_call_common,
                        &args_str,
                        params,
                    ) {
                        return Err(AppSW::Deny);
                    }
                    Ok(())
                }
                Err(_err) => {
                    return Err(AppSW::TxParsingFail);
                }
            }
        }
        Some(_first_byte) => {
            return Err(AppSW::TxParsingFail);
        }
        None => Err(AppSW::TxParsingFail),
    }
}

#[cfg(feature = "speculos")]
pub fn debug_print(args_str: &CappedString<500>, func_call_common: &FunctionCallCommon) {
    func_call_common.debug_print();
    use numtoa::NumToA;

    let mut numtoa_buf = [0u8; 40];

    testing::debug_print("debug printing function call args str  action:\n");
    testing::debug_print("size of self: \n");
    testing::debug_print(core::mem::size_of_val(args_str).numtoa_str(10, &mut numtoa_buf));
    testing::debug_print("\n");
    testing::debug_print("debug printing function call args str  action finish:\n");
}
