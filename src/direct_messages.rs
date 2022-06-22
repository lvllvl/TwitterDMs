use egg_mode; 
use egg_mode::tweet::TweetSource; 
use egg_mode::direct::DirectMessage; 
use chrono::{ DateTime, Utc }; 

pub struct Messages {

    pub message_id: u64,
    pub created_at: DateTime<Utc>,
    pub text: String,
    pub sender_id: u64,
    pub sender_screen_name: String,
    pub source_app: Option<TweetSource>,
    pub recipient_id: u64,
    pub recipient_screen_name: String,

}

impl Messages {
    pub fn new( message_id: u64, created_at: DateTime<Utc>,
                text: String, sender_id: u64,
                sender_screen_name: String, source_app: Option<TweetSource>,
                recipient_id: u64, recipient_screen_name: String  ) -> Self {

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
