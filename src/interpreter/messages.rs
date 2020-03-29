use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
enum ChatType {
    #[serde(rename = "private")]
    Private,
}

impl Default for ChatType {
    fn default() -> Self {
        ChatType::Private
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Chat {
    #[serde(default)]
    first_name: String,
    id: i32,
    #[serde(default)]
    last_name: String,
    #[serde(rename = "type")]
    chat_type: ChatType,
    #[serde(default)]
    username: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct MessageOriginator {
    first_name: String,
    id: i32,
    is_bot: bool,
    language_code: String,
    last_name: String,
    username: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Message {
    chat: Chat,
    date: u64,
    #[serde(default)]
    from: MessageOriginator,
    #[serde(default)]
    entities: Vec<MessageEntity>,
    message_id: i32,
    #[serde(default)]
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum EntityType {
    #[serde(rename = "mention")]
    Mention,
    #[serde(rename = "hashtag")]
    Hashtag,
    #[serde(rename = "cashtag")]
    Cashtag,
    #[serde(rename = "bot_command")]
    BotCmd,
    #[serde(rename = "url")]
    Url,
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "phone_number")]
    PhoneNumber,
    #[serde(rename = "bold")]
    Bold,
    #[serde(rename = "italic")]
    Italic,
    #[serde(rename = "underline")]
    Underline,
    #[serde(rename = "strikethrough")]
    Strikethrough,
    #[serde(rename = "code")]
    Code,
    #[serde(rename = "pre")]
    Pre,
    #[serde(rename = "text_link")]
    TextLink,
    #[serde(rename = "text_mention")]
    TextMention,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct MessageEntity {
    #[serde(rename = "type")]
    entity_type: Option<EntityType>,
    offset: i32,
    length: usize,
    #[serde(default)]
    url: String,
    //user: User,
    #[serde(default)]
    language: String,
}

impl Message {
    pub fn reply(&self, text: String) -> SendMessage {
        SendMessage::new(self.chat.id, text)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SendMessage {
    method: String,
    chat_id: i32,
    text: String,
}

impl SendMessage {
    fn new(chat_id: i32, text: String) -> Self {
        Self {
            method: "sendMessage".to_owned(),
            chat_id,
            text,
        }
    }
}
