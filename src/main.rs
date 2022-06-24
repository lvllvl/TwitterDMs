use mini_redis::{ client, Result };
// use egg_mode::{ Response };
// use futures::stream::TryStreamExt;

mod users; 
mod config; 
mod twitter_api; 
mod direct_messages;
mod error;  

#[ tokio::main ]
async fn main() -> Result<()> {
    
    let request_auth = twitter_api::requesting_user_authorization(); 
    let request_auth: std::result::Result< users::Users, Box<dyn std::error::Error>> = request_auth.await;
    let request_auth = request_auth.unwrap(); // Get token, user_id, screen_name, sqlite connection

    // println!( "{:?}", request_auth ); 
    let _messages = twitter_api::get_direct_messages( &request_auth ).await;  // Get all messages
    // twitter_api::send_DM(String::from( "2 This is a test" ), request_auth.user_id, &request_auth).await; // Send a DM
    // let twitter_user = twitter_api::get_account_by_id( request_auth.user_id,  &request_auth ).await; // test unwrap Twitter User struct  
    Ok(())
}