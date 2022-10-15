pub struct MessageParts {
    pub interface: String,
    pub member: String,
    pub object: String,

    pub string1: String,
    pub string2: String,
    pub int1: u64,
    pub int2: u64,

    pub dict: std::collections::HashMap<String, i32>,
    pub int_array: Vec<u64>,
    pub string_array: Vec<String>,

    pub repeat: usize,
}

pub fn run<F, V>(mut f: F)
    where
        F: FnMut(&MessageParts, bool) -> Option<V>,
        V: core::ops::Deref<Target = [u8]>,

{
    let mixed_parts = make_mixed_message();
    match f(&mixed_parts, true) {
        Some(_) => panic!("message was not sent"),
        None => (),
    };
    let msg = f(&mixed_parts, false).unwrap();
    eprintln!("{}", msg.len());
    let big_array_parts = make_big_array_message();
    let msg = f(&big_array_parts, false).unwrap();
    eprintln!("{}", msg.len());
    let big_str_array_parts = make_big_string_array_message();
    let msg = f(&big_str_array_parts, false).unwrap();
    eprintln!("{}", msg.len());
}

fn make_mixed_message() -> MessageParts {
    let mut dict = std::collections::HashMap::new();
    dict.insert("A".to_owned(), 1234567i32);
    dict.insert("B".to_owned(), 1234567i32);
    dict.insert("C".to_owned(), 1234567i32);
    dict.insert("D".to_owned(), 1234567i32);
    dict.insert("E".to_owned(), 1234567i32);

    MessageParts {
        string1: "Testtest".to_owned(),
        string2: "TesttestTestest".to_owned(),
        int1: 0xFFFFFFFFFFFFFFFFu64,
        int2: 0xFFFFFFFFFFFFFFFFu64,

        int_array: vec![
            0xFFFFFFFFFFFFFFFFu64,
            0xFFFFFFFFFFFFFFFFu64,
            0xFFFFFFFFFFFFFFFFu64,
            0xFFFFFFFFFFFFFFFFu64,
            0xFFFFFFFFFFFFFFFFu64,
            0xFFFFFFFFFFFFFFFFu64,
            0xFFFFFFFFFFFFFFFFu64,
            0xFFFFFFFFFFFFFFFFu64,
            0xFFFFFFFFFFFFFFFFu64,
            0xFFFFFFFFFFFFFFFFu64,
            0xFFFFFFFFFFFFFFFFu64,
            0xFFFFFFFFFFFFFFFFu64,
            0xFFFFFFFFFFFFFFFFu64,
            0xFFFFFFFFFFFFFFFFu64,
            0xFFFFFFFFFFFFFFFFu64,
        ],
        string_array: vec!["".into()],
        dict,
        interface: "io.killing.spark".into(),
        member: "TestSignal".into(),
        object: "/io/killing/spark".into(),
        repeat: 10,
    }
}
fn make_big_array_message() -> MessageParts {
    let mut dict = std::collections::HashMap::new();
    dict.insert("A".to_owned(), 1234567i32);
    let mut int_array = Vec::new();
    int_array.resize(1024 * 10, 0u64);

    MessageParts {
        string1: "Testtest".to_owned(),
        string2: "TesttestTestest".to_owned(),
        int1: 0xFFFFFFFFFFFFFFFFu64,
        int2: 0xFFFFFFFFFFFFFFFFu64,

        int_array,
        string_array: vec!["".into()],
        dict,
        interface: "io.killing.spark".into(),
        member: "TestSignal".into(),
        object: "/io/killing/spark".into(),
        repeat: 1,
    }
}
fn make_big_string_array_message() -> MessageParts {
    let mut dict = std::collections::HashMap::new();
    dict.insert("A".to_owned(), 1234567i32);
    let mut string_array = Vec::new();
    for idx in 0..1024 * 10 {
        string_array.push(format!(
            "{}{}{}{}{}{}{}{}{}{}{}{}",
            idx, idx, idx, idx, idx, idx, idx, idx, idx, idx, idx, idx
        ))
    }

    MessageParts {
        string1: "Testtest".to_owned(),
        string2: "TesttestTestest".to_owned(),
        int1: 0xFFFFFFFFFFFFFFFFu64,
        int2: 0xFFFFFFFFFFFFFFFFu64,

        string_array,
        int_array: vec![0],
        dict,
        interface: "io.killing.spark".into(),
        member: "TestSignal".into(),
        object: "/io/killing/spark".into(),
        repeat: 1,
    }
}
