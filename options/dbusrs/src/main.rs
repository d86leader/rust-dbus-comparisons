mod bench_dbusrs;
pub use common::MessageParts;

fn make_marshalled(x: &MessageParts, b: bool) -> Option<Vec<u8>> {
    let message = bench_dbusrs::make_dbusrs_message(x, b)?;
    let mut buf = Vec::new();
    let marshal = |bytes: &[u8]| {
        buf.extend_from_slice(bytes);
        Ok::<(), ()>(())
    };
    message.marshal(marshal).unwrap();
    Some(buf)
}

fn main() {
    common::run(make_marshalled)
}
