mod bench_dbus_bytestream;
pub use common::MessageParts;

fn main() {
    common::run(bench_dbus_bytestream::make_dbus_bytestream_message)
}
