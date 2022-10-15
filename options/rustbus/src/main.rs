mod bench_rustbus;
pub use common::MessageParts;

fn main() {
    common::run(bench_rustbus::make_rustbus_message)
}
