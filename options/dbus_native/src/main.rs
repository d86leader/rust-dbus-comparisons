mod bench_dbus_native;
pub use common::MessageParts;

fn main() {
    common::run(bench_dbus_native::make_dbus_native_message)
}
