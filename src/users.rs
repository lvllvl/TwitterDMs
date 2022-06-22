use egg_mode; 
use rusqlite; 

#[derive(Debug)]
pub struct Users {
    pub token: egg_mode::auth::Token,
    pub user_id : u64,
    pub screen_name: String,
    pub sqlite_connection: rusqlite::Connection, 
}

impl Users {

    /// Create a new user 
    pub fn new( token: egg_mode::auth::Token, 
                user_id: u64, 
                screen_name: String, 
                sqlite_connection: rusqlite::Connection ) -> Self {
        Users {
            token: token,
            user_id: user_id,
            screen_name: screen_name, 
            sqlite_connection: sqlite_connection, 
        }
    }

    // /// Add a new row to the database
    // pub fn add_row( sender_id: u64, recipient_id: u64, message_id: u64, mtext: String, connection: sqlite::Connection ) {


    // }
}
