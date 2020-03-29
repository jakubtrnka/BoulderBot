mod interpreter;

pub fn process_push_notification(raw_msg: Vec<u8>) -> String {
    // let val: serde_json::Value = serde_json::from_slice(&raw_msg).unwrap_or_default();
    // println!("{}", serde_json::ser::to_string_pretty(&val).unwrap_or_default());
    interpreter::interpret(raw_msg)
}
