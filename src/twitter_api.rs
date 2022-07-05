// use egg_mode::Token;
use egg_mode::user::{UserID, TwitterUser};
// use egg_mode::tweet::Tweet;
// use egg_mode::direct::{ DraftMessage, DirectMessage }; 
use rusqlite::{ Connection, Result, params }; 
// use rusqlite::NO_PARAMS; 

// use http::{Response}; 
use std::{io, collections::HashMap}; 
use crate::config::Config;
use crate::users::Users;
// use crate::error; 
use crate::direct_messages::Messages; 

/// Ask user to authorize this application. Save token, user_id, and screen_name as global variables.  
/// Start the database
pub async fn requesting_user_authorization( connection: &rusqlite::Connection ) -> Result<Users, Box< dyn std::error::Error>> {
    
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
    // let connection = rusqlite::Connection::open("direct_messages.db")?; // Create a SQLite table
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
                    convo_id INTEGER,
                    message_text TEXT,
                    PRIMARY KEY (sender_id, recipient_id, message_id ));
            ",
            [],
        )
        .unwrap();

    let mut _user_info = Users::new( token, user_id, screen_name ); 
    Ok( _user_info )
}

use std::any::type_name; 
/// Check type name of a variable 
pub fn _type_of<T>( _: T ) -> &'static str {
    type_name::<T>()
}

/// Get most recent 50 direct messages associated with a specific user
/// from the last 30 days. FIXME: Find out how to get more messages from a user's account.
pub async fn get_direct_messages( user_token: &Users, connection: &Connection, message_list: &mut HashMap< u64, u64>  ) -> Result<()> {

    // let mut friends_screen_names = HashMap::new();
    // friends_screen_names.insert( user_token.user_id,  &user_token.screen_name ); 
    let timeline = egg_mode::direct::list( &user_token.token ).with_page_size( 50 ); // Get all message, org'd by conversation 
       
    // HashMap<key,value>, key = Unique convo, value = arr[ messages from convo ] arr[0] == Newest message
    let messages = timeline.into_conversations().await.unwrap();

    // Iterate over hashMap keys, sub-loop iterates over messages ( an array of DirectMessage structs )
    for ( _key, val ) in messages { // Add messages to database

        for ( _pos, e ) in val.into_iter().enumerate() { 

            let s_name: String = get_account_by_id( e.sender_id, &user_token ).await.screen_name; // Get sender information 
            let recipient_name: String = get_account_by_id( e.recipient_id, &user_token ).await.screen_name; 
            let convo_id = _key.clone();
            let msg_id = e.id.clone(); 
            
            message_list.insert( msg_id, 1 );  // Add a copy of all message IDs into HashMap
            let m = Messages {
                message_id: e.id.to_string(),
                created_at: e.created_at,
                sender_id: e.sender_id,
                recipient_id: e.recipient_id,
                sender_screen_name: s_name, 
                recipient_screen_name: recipient_name,
                conversation_id: convo_id, 
                text: e.text.clone(),
            };
            
            connection.execute(
                "INSERT INTO direct_messages (
                    message_id, created_at, sender_id, recipient_id, sender_sn, recipient_sn, convo_id, message_text) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                    params![ 
                        &m.message_id, // message_id 
                        &m.created_at, // created_at  
                        &m.sender_id, // sender_id
                        &m.recipient_id, // recipient_id 
                        &m.sender_screen_name, // Screen name
                        &m.recipient_screen_name, // recipient screen name 
                        &_key, // convo_id 
                        &m.text, // message text  
                        ])?;
        }
    }
    Ok(())
}

/// Update messages periodically within your database in order to update your UI
/// Use the egg_mode::into_conversations method to retrieve all recent messages.
/// Messages are returned from newest to oldest. 
/// Check message IDs to see if the message already exists in DB. If yes, then break and end function.
pub async fn update_direct_messages( user_token: &Users, connection: &Connection, message_list: &mut HashMap< u64, u64>  ) -> Result<()> {
    
    // let mut friends_screen_names = HashMap::new();
    // friends_screen_names.insert( user_token.user_id,  &user_token.screen_name ); 
    let timeline = egg_mode::direct::list( &user_token.token ).with_page_size( 50 ); // Get all message, org'd by conversation 
    // HashMap<key,value>, key = Unique convo, value = arr[ messages from convo ] arr[0] == Newest message
    let messages = timeline.into_conversations().await.unwrap();

    for ( _key, val ) in messages { // Add messages to database
        for ( _pos, e ) in val.into_iter().enumerate() { 

            let s_name: String = get_account_by_id( e.sender_id, &user_token ).await.screen_name; // Get sender information 
            let recipient_name: String = get_account_by_id( e.recipient_id, &user_token ).await.screen_name; 
            let convo_id = _key.clone();
            let msg_id = e.id.clone(); 
           
            if message_list.contains_key( &msg_id ) { // Check if message already exists in SQLite DB
                println!( "Message found within database already. Breaking this loop.");
                break
            } else {
                message_list.insert( msg_id, 1 );  // Add a copy of all message IDs into HashMap
                let m = Messages {
                    message_id: e.id.to_string(),
                    created_at: e.created_at,
                    sender_id: e.sender_id,
                    recipient_id: e.recipient_id,
                    sender_screen_name: s_name, 
                    recipient_screen_name: recipient_name,
                    conversation_id: convo_id, 
                    text: e.text.clone(),
                };

                connection.execute(
                    "INSERT INTO direct_messages (
                        message_id, created_at, sender_id, recipient_id, sender_sn, recipient_sn, convo_id, message_text) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                        params![ 
                            &m.message_id, // message_id 
                            &m.created_at, // created_at  
                            &m.sender_id, // sender_id
                            &m.recipient_id, // recipient_id 
                            &m.sender_screen_name, // Screen name
                            &m.recipient_screen_name, // recipient screen name 
                            &_key, // convo_id 
                            &m.text, // message text  
                            ])?;
            }
        }
    }
    Ok(())
}



/// Gets an account object by username.
pub async fn _get_account_by_name( username: String, user_token: &Users ) -> TwitterUser {

    let user = egg_mode::user::show( username, &user_token.token ).await.unwrap();
    return user.response;
}

/// Gets an account object by user ID.
pub async fn get_account_by_id( user_id: u64, user_token: &Users ) -> TwitterUser {

    let user = egg_mode::user::show(user_id, &user_token.token ).await.unwrap();
    return user.response;
}

/// Send a Tweet, currently sends TEXT ONLY 
pub async fn _send_tweet( text: String, user_token: &Users ) {
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
pub async fn _send_dm( text: String, recipient_id: u64, user_token: &Users, connection: &Connection ) -> Result<()> {
   
    // Include the conversation id in the signature !!! 
    let _message = egg_mode::direct::DraftMessage::new( text, UserID::ID(recipient_id)).send( &user_token.token ).await;
    for m in _message {

        let recipient_name: String = get_account_by_id( m.recipient_id, &user_token ).await.screen_name; 
        let sender_screen_name = user_token.screen_name.clone(); 
        let msg_text = m.text.clone(); 
        
        // Get conversation Id ... IF it exists 
        let convo_id2: u64 = get_convo_id_by_recipient_id( &user_token,  recipient_id, connection ).unwrap();

        let one_msg = Messages::_new( 
            m.id.to_string(),
            m.created_at, 
            user_token.user_id,
            recipient_id,
            sender_screen_name.to_string(),
            recipient_name,
            convo_id2,
            msg_text,
        );

        // Insert new convo into database 
        insert_new_message_db( one_msg , &connection ).unwrap()  
                    } 
        Ok(())
}

/// Get a Message struct and insert the new message into the SQLite database
/// FIXME: currently only works for SENT message only, not recieved messages 
pub fn insert_new_message_db( one_msg: Messages, connection: &Connection) -> Result<()> {

        connection.execute( 
            "INSERT INTO direct_messages (message_id, created_at, sender_id, recipient_id, sender_sn, recipient_sn, convo_id, message_text)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)", 
            params![    &one_msg.message_id, 
                        &one_msg.created_at, 
                        &one_msg.sender_id, 
                        &one_msg.recipient_id, 
                        &one_msg.sender_screen_name, 
                        &one_msg.recipient_screen_name, 
                        &one_msg.conversation_id, 
                        &one_msg.text])?;
                     
        Ok(())
}

/// Get conversation ID by search for a row with the recipient ID / recipient screen name 
/// FIXME: Take into account that the conversation may NOT exist, i.e., user starts a NEW conversation
pub fn get_convo_id_by_recipient_id( user_token: &Users, recipient_id: u64, connection: &Connection ) -> Result<u64> {
    
    // let r_id: u64 = recipient_id; 

    let mut stmt = connection.prepare( 
        "SELECT * FROM direct_messages WHERE recipient_id=:recipient_id;")?;
    // let message_iter = stmt.query_map( params![":recipient_id", recipient_id.to_string().as_str()], | row | {
    let message_iter = stmt.query_map( params![ recipient_id.to_string().as_str() ], | row | {

        Ok( Messages{
                message_id: row.get( 0 )?,
                created_at: row.get( 1 )?,
                sender_id: row.get( 2 )?,
                recipient_id: row.get( 3 )? ,
                sender_screen_name: row.get( 4 )?,
                recipient_screen_name: row.get( 5 )?,
                conversation_id: row.get( 6 )?,
                text: row.get( 7 )?,
        })
    })?; 

    let mut ans = Vec::new(); 
    for message in message_iter {
        for m in message {
            ans.push( m.conversation_id );
            break
        }
        break
    }

    Ok( ans[0] )
}

/// Get all unique screen names in the SQLite database, return a vector 
pub fn get_all_screen_names_from_dms( user_token: &Users, connection: &Connection ) -> Result< Vec<String> > {

    // Check all of the sender screen names
    let mut stmt = connection.prepare( 
        "SELECT sender_sn FROM direct_messages")?;
    let message_iter = stmt.query_map( [], | row | {

        Ok( Messages{
                message_id: row.get( 0 )?,
                created_at: row.get( 1 )?,
                sender_id: row.get( 2 )?,
                recipient_id: row.get( 3 )? ,
                sender_screen_name: row.get( 4 )?,
                recipient_screen_name: row.get( 5 )?,
                conversation_id: row.get( 6 )?,
                text: row.get( 7 )?,
        })
    })?; 
    
    // Check all of recipient screen names 
    let mut stmt_2 = connection.prepare( 
        "SELECT recipient_sn FROM direct_messages")?;
    let recipient_iter = stmt_2.query_map( [], | row | {

        Ok( Messages{
                message_id: row.get( 0 )?,
                created_at: row.get( 1 )?,
                sender_id: row.get( 2 )?,
                recipient_id: row.get( 3 )? ,
                sender_screen_name: row.get( 4 )?,
                recipient_screen_name: row.get( 5 )?,
                conversation_id: row.get( 6 )?,
                text: row.get( 7 )?,
        })
    })?;

    let mut hash_ans = HashMap::new(); // Create a hash map, collect unique keys only 
    for mess in message_iter { // iterate over sender screen names 
        for m in mess {
            hash_ans.insert( m.sender_screen_name, 1 ); 
        }
    }

    for recipient in recipient_iter { // iterate over recipient screen names
        for r in recipient {
            hash_ans.insert( r.recipient_screen_name, 1 ); 
        }
    }

    // Conver HashMap to vector 
    let ans = hash_ans.keys().cloned().collect::<Vec<String>>();
    Ok( ans )
}
