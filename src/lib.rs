mod interpreter;

pub fn process_push_notification(raw_msg: Vec<u8>) -> String {
    if let Ok(update) = serde_json::from_slice::<interpreter::WebHookMsg>(&raw_msg) {
        let mut ctr = 0;
        for char in update.message.text.chars() {
            if char == 'a' || char == 'A' {
                ctr += 1;
            }
        }
        let x = format!("počet áček je: {}", ctr);
        let resp = update.message.reply(x);
        serde_json::to_string(&resp).unwrap_or_default()
    } else {
        dbg!("chyba");
        String::new()
    }
}
