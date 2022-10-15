mod bench_dbus_message_parser;
pub use common::MessageParts;

fn main() {
    common::run(bench_dbus_message_parser::make_dbus_message_parser_message)
}
