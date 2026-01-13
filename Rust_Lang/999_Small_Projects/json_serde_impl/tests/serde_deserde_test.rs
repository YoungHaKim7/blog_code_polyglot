use json_serde_impl::*;

#[test]
fn serde_data() {
    let raw_json = RawJsonSerde {
        title: "Hello \"World\"".to_string(),
        value: "Test\\Value".to_string(),
    };

    let json_string = raw_json.to_json();
    assert!(json_string.contains("title"));
    assert!(json_string.contains("value"));

    let desered = RawJsonDeserde::from_json(&json_string).unwrap();
    assert_eq!(desered.title, "Hello \"World\"");
    assert_eq!(desered.value, "Test\\Value");
}

#[test]
fn serde_simple() {
    let data = RawJsonSerde {
        title: "Test Title".to_string(),
        value: "Test Value".to_string(),
    };

    let json = data.to_json();
    let parsed = RawJsonDeserde::from_json(&json).unwrap();

    assert_eq!(parsed.title, "Test Title");
    assert_eq!(parsed.value, "Test Value");
}

#[test]
fn deserde_invalid_json() {
    let result = RawJsonDeserde::from_json("not json");
    assert!(result.is_err());
}

#[test]
fn input_json() {
    let json_data = include_str!("../assets/input.json");
    let parsed = RawJsonDeserde::from_json(&json_data).unwrap();
    println!("{parsed:#?}");
}
