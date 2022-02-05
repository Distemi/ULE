// Chat's component
#[derive(Debug, Serialize)]
pub struct ChatMessage {
    // Chat's text
    pub text: String,
    // Simple style - bold. Skip to showing if empty
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub bold: String,
    // Child components of the component. Skip to showing if empty
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub extra: Vec<ChatMessage>,
}

impl ChatMessage {
    // Creating a component
    pub fn text(text: String) -> Self {
        Self {
            // Replacing a formatting from user-like to minecraft-like
            text: text.replace("&", "§"),
            // Empty style
            bold: String::new(),
            // Empty child components
            extra: vec![],
        }
    }
    // Updating bold style
    pub fn set_bold(&mut self, value: bool) {
        self.bold = value.to_string();
    }
    // Creating component from str ( not String )
    pub fn str(text: &str) -> Self {
        ChatMessage::text(String::from(text).replace("&", "§"))
    }
}
