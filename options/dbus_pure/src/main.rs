mod bench_dbus_pure;
pub use common::MessageParts;

fn main() {
    common::run(bench_dbus_pure::make_dbus_pure_message)
}
