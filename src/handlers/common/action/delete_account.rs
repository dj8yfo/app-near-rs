use crate::sign_ui;
use crate::sign_ui::widgets::check_display_error;
use crate::{
    parsing::{types::DeleteAccount, HashingStream, SingleTxStream},
    AppSW,
};

use super::ActionParams;

pub fn handle(
    stream: &mut HashingStream<SingleTxStream<'_>>,
    params: ActionParams,
) -> Result<(), AppSW> {
    let mut delete_account = DeleteAccount::new();
    delete_account
        .deserialize_reader_in_place(stream)
        .map_err(|_err| AppSW::TxParsingFail)?;

    if !sign_ui::action::ui_display_delete_account(&mut delete_account, params)
        .map_err(check_display_error)?
    {
        return Err(AppSW::Deny);
    }
    Ok(())
}
