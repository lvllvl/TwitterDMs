use egg_mode; 
use std::{rc::Rc, collections::HashMap}; 

#[derive(Debug, Clone)]
pub struct Users {
    pub token: egg_mode::auth::Token,
    pub user_id : u64,
    pub screen_name: Rc<String>,
    // pub friends_list: HashMap< u64, String > 
    // pub sqlite_connection: rusqlite::Connection,
}

impl Users {

    /// Create a new user 
    pub fn new( token: egg_mode::auth::Token, 
                user_id: u64, 
                screen_name: String, 
                // sqlite_connection: rusqlite::Connection,
            ) -> Self {
        Users {
            token: token,
            user_id: user_id,
            screen_name: Rc::new( screen_name ) , 
            // sqlite_connection: sqlite_connection,
        }
    }
}
