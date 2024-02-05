use ledger_device_sdk::ui::{
    bitmaps::{CROSSMARK, EYE, VALIDATE_14},
    gadgets::{Field, MultiFieldReview},
};

use crate::{
    app_ui::fields_writer::FieldsWriter, parsing::types::nep413::payload::Payload,
    utils::types::elipsis_fields::ElipsisFields,
};

struct FieldsContext {
    nonce_buffer: [u8; 64],
}

impl FieldsContext {
    pub fn new() -> Self {
        Self {
            nonce_buffer: [0u8; 64],
        }
    }
}

fn format<'b, 'a: 'b>(
    payload: &'b Payload,
    field_context: &'a mut FieldsContext,
    writer: &'_ mut FieldsWriter<'b, 7>,
) {
    // 2
    let message_fields = payload.message.ui_fields("Message");
    writer.push_fields(message_fields).unwrap();

    // 3
    hex::encode_to_slice(&payload.nonce, &mut field_context.nonce_buffer).unwrap();
    writer
        .push_fields(ElipsisFields::one(Field {
            name: "Nonce",
            value: core::str::from_utf8(&field_context.nonce_buffer).unwrap(),
        }))
        .unwrap();

    // 5
    let recipient_fields = payload.recipient.ui_fields("Recipient");
    writer.push_fields(recipient_fields).unwrap();

    // 7
    if let Some(callback_url) = payload.callback_url.as_ref() {
        let callback_url_fields = callback_url.ui_fields("Callback Url");
        writer.push_fields(callback_url_fields).unwrap();
    }
}
pub fn ui_display(payload: &Payload) -> bool {
    #[cfg(feature = "speculos")]
    payload.debug_print();

    let mut field_writer: FieldsWriter<'_, 7> = FieldsWriter::new();
    let mut field_context: FieldsContext = FieldsContext::new();
    format(payload, &mut field_context, &mut field_writer);

    let my_review = MultiFieldReview::new(
        field_writer.get_fields(),
        &["View NEP413 msg sign"],
        Some(&EYE),
        "Sign",
        Some(&VALIDATE_14),
        "Reject",
        Some(&CROSSMARK),
    );

    my_review.show()
}
