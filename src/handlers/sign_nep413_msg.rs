use crate::parsing::types::common::message_discriminant::NEP_413_SIGN_MESSAGE;
use crate::parsing::types::nep413::payload::Payload;
use crate::sign_ui;
use crate::{
    parsing::{types::MessageDiscriminant, HashingStream, SingleTxStream},
    utils::crypto,
    AppSW,
};
use borsh::BorshDeserialize;

use super::common::finalize_sign::{self, Signature};

pub fn handler(mut stream: SingleTxStream<'_>) -> Result<Signature, AppSW> {
    sign_ui::widgets::display_receiving();
    let path = <crypto::PathBip32 as BorshDeserialize>::deserialize_reader(&mut stream)
        .map_err(|_| AppSW::Bip32PathParsingFail)?;

    let mut stream = HashingStream::new(stream)?;

    let msg_discriminant = MessageDiscriminant::new(NEP_413_SIGN_MESSAGE);

    let prefix_bytes = msg_discriminant.borsh_serialize();

    stream
        .feed_slice(&prefix_bytes)
        .map_err(|_err| AppSW::TxParsingFail)?;

    let mut payload: Payload = Payload::new();

    payload
        .deserialize_reader_in_place(&mut stream)
        .map_err(|_err| AppSW::TxParsingFail)?;

    if !sign_ui::nep413::payload::ui_display(&mut payload) {
        return Err(AppSW::Deny);
    }

    finalize_sign::end(stream, &path)
}
