use std::io::Cursor;

use rnbt::read_nbt;
use serde_nbt::*;

fn test_back_and_forth<T: serde::Serialize + serde::de::DeserializeOwned + std::fmt::Debug + PartialEq>(value: T) {
    let mut output = Vec::new();
    to_writer(&mut output, &value, "TestStruct".to_string()).unwrap();
    let expected: T = from_bytes(&output).unwrap();

    assert_eq!(expected, value);
}

#[test]
fn try_simple_struct() {
    #[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
    struct TestStruct {
        a: i32,
        b: String,
    }

    let simple = TestStruct {
        a: 1,
        b: "hello".to_string(),
    };
    let mut output = Vec::new();
    to_writer(&mut output, &simple, "TestStruct".to_string()).unwrap();

    println!("{:?}", output);


    // try as nbt
    //
    let mut cursor = Cursor::new(output.clone());
    let nbt = read_nbt(&mut cursor).unwrap();
    println!("{:?}", nbt);

    let expected: TestStruct = from_bytes(&output).unwrap();


    assert_eq!(expected, simple);
}
