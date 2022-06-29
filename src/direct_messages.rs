use egg_mode; 
use egg_mode::tweet::TweetSource; 
// use egg_mode::direct::DirectMessage; 
use chrono::{ DateTime, Utc }; 

pub struct Messages {

    pub message_id: u64,
    pub created_at: DateTime<Utc>,
    pub sender_id: u64,
    pub recipient_id: u64,
    pub sender_screen_name: String,
    pub recipient_screen_name: String,
    pub conversation_id: u64,
    pub text: String,
}

impl Messages {
    pub fn new( 
                message_id: u64, 
                created_at: DateTime<Utc>,
                sender_id: u64,
                recipient_id: u64, 
                sender_screen_name: String, 
                recipient_screen_name: String, 
                text: String, 
            ) -> Self {

        Messages {
            message_id: message_id,
            created_at: created_at,
            text: text,
            sender_id: sender_id,
            sender_screen_name: sender_screen_name,
            source_app: source_app,
            recipient_id: recipient_id,
            recipient_screen_name: recipient_screen_name,
        }

    }
}

                        // &e.id.to_string(), message ID
                        // &dateTime_var, date Time 
                        // &e.sender_id, sender ID
                        // &e.recipient_id, recipient ID
                        // &s_name, // Screen name 
                        // &recipient_name, // recipient screen name 
                        // &_key.to_string(), // conversation ID 
                        // &text,  // text 