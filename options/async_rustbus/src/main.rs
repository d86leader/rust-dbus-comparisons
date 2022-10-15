mod bench_async_rustbus;
pub use common::MessageParts;

fn main() {
    common::run(bench_async_rustbus::make_async_rustbus_message)
}
