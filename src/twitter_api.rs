// use egg_mode::Token;
use egg_mode::user::{UserID, TwitterUser};
// use egg_mode::tweet::Tweet;
use egg_mode::direct::{ DraftMessage, DirectMessage }; 
use rusqlite::{ Connection, Result, params }; 
use rusqlite::NO_PARAMS; 

use http::{Response}; 
use std::io; 
use crate::config::Config;
use crate::users::Users;
use crate::error; 
// use crate::direct_messages::Messages; 

/// Ask user to authorize this application. Save token, user_id, and screen_name as global variables.  
/// Start the database
pub async fn requesting_user_authorization() -> Result<Users, Box< dyn std::error::Error>> {
    
    let config = Config::new();
    let con_token = egg_mode::KeyPair::new( config.twitter_consumer_key,  config.twitter_consumer_secret ); 
    // // "oob" is needed for PIN-based auth; see docs for 'request_token' for more info 

    let request_token = egg_mode::auth::request_token( &con_token, "oob" ).await.unwrap();  
    let auth_url = egg_mode::auth::authorize_url( &request_token );  // Provide URL to user to generate PIN 
    println!( "Use this URL to authorize this app and get a PIN:\n {:?}", auth_url ); 

    // Capture auth-PIN from user 
    use io::{stdin,stdout,Write};
    let mut verifier = String::new();
    print!("Please enter the authorization number: ");
    let _=stdout().flush();
    stdin().read_line( &mut verifier ).expect("Did not enter a correct string");
    if let Some('\n')=verifier.chars().next_back() {
        verifier.pop();
    }
    if let Some('\r')=verifier.chars().next_back() {
        verifier.pop();
    }
    println!( "You typed: {}", verifier ); //TODO: delete this 
    
    // Return This   
    let ( token, user_id, screen_name ) = 
        egg_mode::auth::access_token( con_token, &request_token, verifier ).await.unwrap(); 
    
    // Create the database SQLite
    // let connection = sqlite::open(":memory:").unwrap(); // Create a SQLite table
    let connection = rusqlite::Connection::open("direct_messages.db")?; // Create a SQLite table
    connection.execute(
            "
            CREATE TABLE IF NOT EXISTS direct_messages 
                (   
                    message_id TEXT,
                    created_at INTEGER,
                    sender_id INTEGER,
                    recipient_id INTEGER,
                    sender_sn TEXT,
                    recipient_sn TEXT,
                    convo_id TEXT,
                    message_text TEXT,
                    PRIMARY KEY (sender_id, recipient_id, message_id ));
            ",
            NO_PARAMS,
        )
        .unwrap();

    // let friends = std::collections::HashMap::new();
    let mut _user_info = Users::new( token, user_id, screen_name, connection); 
    Ok( _user_info )
}

use std::any::type_name; 
/// Check type name of a variable 
pub fn type_of<T>( _: T ) -> &'static str {
    type_name::<T>()
}

/// Get most recent 50 direct messages associated with a specific user
/// from the last 30 days. FIXME: Find out how to get more messages from a user's account.
pub async fn get_direct_messages( user_token: &Users ) -> Result<()> {

    let mut friends_screen_names = std::collections::HashMap::new();
    friends_screen_names.insert( user_token.user_id,  &user_token.screen_name ); 

    let mut timeline = egg_mode::direct::list( &user_token.token ).with_page_size( 50 ); // Get all message, org'd by conversation 
       
    // HashMap<key,value>, key = Unique convo, value = arr[ messages from convo ] arr[0] == Newest message
    let mut messages = timeline.into_conversations().await.unwrap();
    let convo_keys = messages.keys();
    println!( "convo_keys: {:?}", convo_keys ); // TODO: delete this !  

    // Iterate over hashMap keys, sub-loop iterates over messages ( an array of DirectMessage structs )
    for ( _key, val ) in &messages { // Add messages to database
        // println!( "Type of _key: {}", type_of( _key ) ); // TODO: delete this ...  Type = &u64
        for ( pos, e ) in val.iter().enumerate() { 
            
            // FIXME: Create a HashMap for usernames. < K,V > == < user_id, screen_name > 
            let s_name: String = get_account_by_id( e.sender_id, &user_token ).await.screen_name; // Get sender information 
            let recipient_name: String = get_account_by_id( e.recipient_id, &user_token ).await.screen_name; 
            let text: String = e.text.clone();  // message text 
            let dateTime_var: i64 = e.created_at.timestamp(); // Get the dateTime in Unix Time format
            let dateTime_var: u64 = dateTime_var.unsigned_abs(); // Convert to u64 FIXME: Not sure if this destroys / alters date data

            user_token.sqlite_connection.execute(
                "INSERT INTO direct_messages (
                    message_id, created_at, sender_id, recipient_id, sender_sn, recipient_sn, convo_id, message_text) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                    params![ 
                        &e.id.to_string(), 
                        &dateTime_var, 
                        &e.sender_id,
                        &e.recipient_id,
                        &s_name, // Screen name
                        &recipient_name, // recipient screen name 
                        &_key.to_string(),
                        &text,  
                        ])?;
        }
    }
    Ok(())
}


/// Gets an account object by username.
pub async fn get_account_by_name( username: String, user_token: &Users ) -> TwitterUser {

    let user = egg_mode::user::show( username, &user_token.token ).await.unwrap();
    return user.response;
}

/// Gets an account object by user ID.
pub async fn get_account_by_id( user_id: u64, user_token: &Users ) -> TwitterUser {

    let user = egg_mode::user::show(user_id, &user_token.token ).await.unwrap();
    return user.response;
}

/// Send a Tweet, currently sends TEXT ONLY 
pub async fn send_tweet( text: String, user_token: &Users ) {
    let _post = egg_mode::tweet::DraftTweet::new( text ).send( &user_token.token ).await;
}

/// Gets the most recent tweet from the specified user.
pub async fn _get_last_tweet(user_id: u64, user_token: &Users ) -> u64 {

    let timeline = egg_mode::tweet::user_timeline(user_id, false, false, &user_token.token).with_page_size(100);
    let (timeline, _feed) = timeline.older(None).await.unwrap();
    return timeline.max_id.unwrap();
}

/// Recipient must allow DMs from authenticated user for this to be successful. 
///     e.g., Recipient must follow authenticated user,
///           or they must allow DMs from anyone
/// Latter setting has no visibility on API. There may be situations where you are
/// unable to verify the recipient's ability to recieve request DM beforehanbd.
/// FIXME: Do you want to return the same signature as egg_mode? e.g., Result<Response<directMessage>, error>
/// FIXME: This should update your database --> an INSERT statment
pub async fn send_DM( text: String, recipient_id: u64, user_token: &Users ) {
    
    let _message = egg_mode::direct::DraftMessage::new( text, UserID::ID(recipient_id)).send( &user_token.token ).await;
    match _message {
        Err(e) => println!("{}", e ),  
        Ok(_) => println!("Direct Message was sent!" ),   
    };
}