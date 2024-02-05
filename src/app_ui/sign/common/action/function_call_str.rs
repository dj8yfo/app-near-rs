use crate::utils::types::capped_string::CappedString;

use crate::app_ui::fields_writer::FieldsWriter;

pub fn format<'b, 'a: 'b>(args: &'b CappedString<500>, writer: &'_ mut FieldsWriter<'b, 7>) {
    let args_fields = args.ui_fields("Args String");

    writer.push_fields(args_fields).unwrap();
}
