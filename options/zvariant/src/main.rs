mod bench_zvariant;
pub use common::MessageParts;

fn main() {
    common::run(|a1, a2| bench_zvariant::make_zvariant_message(a1, a2).map(|x| Vec::from(x.as_bytes())))
}
