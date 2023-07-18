# Outline

## Project Overview
A Rust project that aims to create a user interface (UI) for accessing / interacting with Twitter direct messages (DMs).

This project accesses the Twitter API using Rust egg_mode. 

This project currently uses a rusqlite database to temporarily store and access DMs. 

UI Example: 

![UI Twitter Image]("./images/ui_twitter_dms.png")

## What needs to be done?

1. Decide display method, e.g., TUI-Rust, webapp?
    + twitter_api.rs :: fn requesting_user_authoriation **must** be changed based on the medium
    + Make this easy for recruiters to USE, easy to UNDERSTAND visually

2. twitter_api.rs :: fn get_direct_messages
    + Current limit on amount of tweets you can get - How can I increase this limit?
    + Decide: We don't need all the info contained in the hashmap, should we create a custom data structure?
    + This data structure would be the same (e.g., HashMap<k,v> = <ConvoIDs, Messages> ) 

3. How to deliver data I already have? Should I store data in a database?

4. React version -> How to deliver data from Rust HashMap to JavaScript? 
    + Option 1: Output HashMap to JSON - import with JavasScript

5. **VIP** Adding new conversations
    + Find the user
    + Verify if there was any previous chat history, if yes, import chats. If not, create new chat.

6. Sending a message to someone
    + Get user text-message input 
    + Send user message

7. Recieving messages / Updating chat history 
    + How to do this efficiently without hitting API call limit?
    + Do you have to pay to increase limit?




## What's the MVP? Just ship a project! 
### GET
    [] Messages on start up and insert into User Interface (UI)
    [] Messages periodically, continually check for updates 

### POST 
    [] Send messages within EXISTING conversations and update UI
        [] Send a message
        [] Update a conversation 

### Testing !

    [ ] ADD THIS ! 
