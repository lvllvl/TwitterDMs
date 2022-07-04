use mini_redis::{ client, Result };
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::collections::HashMap;
use futures::executor::block_on; 
use chrono::{DateTime, Utc};
use serde::{ Deserialize, Serialize }; 
use rusqlite::{ Connection, NO_PARAMS } ; 
use std::{
    error:: { Error }, 
    io, sync::mpsc, 
    fs, 
    time::{ 
        Duration, Instant }, 
    thread };

use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style },
    widgets::{
        Block, Borders, BorderType, Cell, List, ListItem, ListState, Paragraph, Row, Table, Tabs
    },
    text::{ Span, Spans },
    Frame, Terminal,
};

// use egg_mode::{ Response };
// use futures::stream::TryStreamExt;
mod users; 
mod config; 
mod twitter_api; 
mod direct_messages;

// FIXME: Delete the database after each session
// const DB_PATH: &str = "./direct_messages.db";

// Data structure for input events
enum Event<I> {
    Input(I),
    Tick,
}
#[derive( Copy, Clone, Debug ) ]
enum MenuItem {
    Home,
    DMs,
    EnterTextMessage,
    Tweeting,
}
impl From<MenuItem> for usize {
    fn from( input: MenuItem ) -> usize {
        match input {
            MenuItem::Home => 0,
            MenuItem::DMs => 1, 
            MenuItem::EnterTextMessage => 2, 
            MenuItem::Tweeting => 3, 
        }
    }
}

#[ tokio::main ]
async fn main() -> Result<()> {

    let connection = rusqlite::Connection::open("direct_messages.db")?; // Create a SQLite table
    let mut friends_list = HashMap::new();
    let mut message_list: HashMap<u64, u64> = HashMap::new();
    message_list.insert( 123456, 123456 ); // <k, v> == message_id, count_num, keep track of all messages recieved 
    
    // Get authorization, then organize messages   
    let request_auth = twitter_api::requesting_user_authorization( &connection ); 
    let request_auth: std::result::Result< users::Users, Box<dyn std::error::Error>> = request_auth.await;
    let request_auth = request_auth.unwrap(); // Get token, user_id, screen_name, sqlite connection

    friends_list.insert( request_auth.user_id.clone(), request_auth.screen_name.clone() ); 

    let _messages = twitter_api::get_direct_messages( &request_auth, &connection, &mut message_list ).await;  // Get all messages
    let _dms_list = read_db( &connection );  // Get all messages from database

    // TODO: Convert this to a test 
    // let test_dm = direct_messages::Messages::_new( 
    //    "1234452456".to_string(), 
    //    chrono::prelude::Utc::now(), 
    //    012332423, 
    //    34509425, 
    //    "fake_sender".to_string(), 
    //    "fake_recipient".to_string(), 
    //    1234567, 
    //    "FAKE FAKE FAKE FAKE!!!".to_string());
    // println!( "INSERT == {:?}", twitter_api::insert_new_message_db( test_dm ,  &request_auth ).unwrap() ) ;


    // FIXME: get conversation ID, send the convo_id in the signature 
    twitter_api::_send_dm(String::from( "1:41, Monday another test!" ), request_auth.user_id, &request_auth, &connection ).await.unwrap(); // Send a DM
    twitter_api::update_direct_messages( &request_auth, &connection, &mut message_list).await.unwrap(); 

    // Setup Terminal
    // enable_raw_mode().expect("Can run in raw mode");
    // let mut stdout = io::stdout();
    // execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    // let backend = CrosstermBackend::new(stdout);
    // let mut terminal = Terminal::new(backend)?;
    // terminal.clear()?; // clear the terminal and force full redraw on next draw call 
    
    // create app and run it
    // let res = run_app(&mut terminal);

    // // restore terminal
    // disable_raw_mode()?;
    // execute!(
    //     terminal.backend_mut(),
    //     LeaveAlternateScreen,
    //     DisableMouseCapture
    // )?;
    // terminal.show_cursor()?;

    // if let Err(err) = res {
    //     println!("{:?}", err)
    // }

    // FIXME: Start here 
    // let menu_title = vec![ "home", "pets", "add", "delete", "quit" ];
    // let mut active_menu_item = MenuItem::Home;
    // let mut pet_list_state = ListState::default();
    // pet_list_state.select( Some( 0 ) );

    // let (tx, rx) = mpsc::channel();
    // let tick_rate = Duration::from_millis(200);
    // thread::spawn(move || {
    //     let mut last_tick = Instant::now();
    //     loop {
    //         let timeout = tick_rate
    //             .checked_sub(last_tick.elapsed())
    //             .unwrap_or_else(|| Duration::from_secs(0));

    //         if event::poll(timeout).expect("poll works") {
    //             if let CEvent::Key(key) = event::read().expect("can read events") {
    //                 tx.send(Event::Input(key)).expect("can send events");
    //             }
    //         }

    //         if last_tick.elapsed() >= tick_rate {
    //             if let Ok(_) = tx.send(Event::Tick) {
    //                 last_tick = Instant::now();
    //             }
    //         }
    //     }
    // });

    //     // Tutorial loop TODO: delete this 
    // loop {
    //     terminal.draw( |rect| {
    //         let size = rect.size();
    //         let chunks = Layout::default()
    //             .direction( Direction::Horizontal )
    //             .constraints( 
    //                 [
    //                     Constraint::Percentage( 30 ),
    //                     Constraint::Percentage( 70 ), 
    //                 ]
    //                 .as_ref(),
    //             )
    //             .split( size );

    //         let right_chunks = Layout::default()
    //             .direction( Direction::Vertical )
    //             .constraints( 
    //                 [
    //                     Constraint::Percentage( 80 ),
    //                     Constraint::Percentage( 20 ),
    //                 ]
    //                 .as_ref(),
    //             )
    //             .split( chunks[ 1 ] );
            
    //         let block = Block::default().title("DMs").borders(Borders::ALL);
    //         rect.render_widget(block, chunks[0]);
        
    //         let block = Block::default().title("Messages").borders(Borders::ALL);
    //         rect.render_widget(block, right_chunks[0]);
            
    //         let block = Block::default().title("Enter Text Below").borders(Borders::ALL)
    //             .style( Style::default().fg( Color::LightRed ))
    //             .borders( Borders::ALL )
    //             .style( Style::default().fg( Color::White ))
    //             .border_type( BorderType::Thick );
    //         rect.render_widget(block, right_chunks[1]);
    //        // TODO: get message here...
    //     //    let copyright = Paragraph::new("Twitter DMs CLI")
    //     //         .style(Style::default().fg(Color::LightCyan)) // FIXME: change the colors here 
    //     //         .alignment(Alignment::Center)
    //     //         .block(
    //     //             Block::default()
    //     //                 .borders(Borders::ALL)
    //     //                 .style(Style::default().fg(Color::White))
    //     //                 .title("") // FIXME: Change the title...  or not.
    //     //                 .border_type(BorderType::Plain),
    //     //         ); 

    //     //    let menu = menu_title
    //     //         .iter()
    //     //         .map( |t| {
    //     //             let (first, rest ) = t.split_at( 1 );
    //     //             Spans::from( vec![
    //     //                 Span::styled( first,
    //     //                     Style::default()
    //     //                         .fg( Color::Yellow )
    //     //                         .add_modifier( Modifier::UNDERLINED ),
    //     //                     ),
    //     //                     Span::styled(rest, Style::default().fg( Color::White)),
    //     //             ])
    //     //         })
    //     //         .collect();
            
    //             // let tabs = Tabs::new( menu )
    //             //     .select( active_menu_item.into() )
    //             //     .block( Block::default().title("Menu").borders( Borders::ALL))
    //             //     .style( Style::default().fg( Color::White ))
    //             //     .highlight_style( Style::default().fg( Color::Yellow ))
    //             //     .divider( Span::raw( "|" ));
                
    //             // rect.render_widget( tabs, chunks[0] );
                
    //             // TODO: This should reflect the selcted conversation 
    //             // match active_menu_item {
    //             //     MenuItem::Home => rect.render_widget( render_home(), chunks[1] ),
    //             //     MenuItem::DMs => {
    //             //         let pets_chunks = Layout::default()
    //             //             .direction( Direction::Horizontal )
    //             //             .constraints( 
    //             //                 [ Constraint::Percentage( 20 ), Constraint::Percentage( 80 )].as_ref(),
    //             //             )
    //             //             .split( chunks[1] );
    //             //         let ( left, right ) = render_pets( &pet_list_state ); 
    //             //         rect.render_stateful_widget( left, pets_chunks[0], &mut pet_list_state ); 
    //             //         rect.render_widget( right, pets_chunks[1] ); 
    //             //     }
    //             // }
    //             // rect.render_widget( copyright, chunks[2] );  
    //     })?;

    //     // TODO: Keyboard monitoring --> take the following actions 
    //     match rx.recv()? {
    //         Event::Input(event) => match event.code {
    //             KeyCode::Char('q') => {
    //                 disable_raw_mode()?;
    //                 terminal.show_cursor()?;
    //                 break;
    //             }
    //             KeyCode::Char('d') => active_menu_item = MenuItem::DMs,
    //             KeyCode::Char('t') => active_menu_item = MenuItem::Tweeting,
    //             KeyCode::Char('e') => active_menu_item = MenuItem::EnterTextMessage,
    //             _ => {}
    //         },
    //         Event::Tick => {}
    //     }
    //     // read_db( &request_auth.sqlite_connection ).unwrap(); 
    // }
    // FIXME: End here 
    Ok(())
}


/// Pull message information from the database 
fn read_db( conn: &Connection ) -> Result<Vec<direct_messages::Messages>> {
    // println!( "read_db function starting...\n");
    let mut stmt = conn.prepare( "SELECT * FROM direct_messages ORDER BY convo_id")?;
    let message_iter = stmt.query_map( [], | row | {

        Ok( direct_messages::Messages{
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

    let mut dms = Vec::new();
    for message in message_iter {
        dms.push( message? );
    }

    Ok( dms ) 
}

fn _run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        terminal.draw(|f| _ui(f))?;

        if let CEvent::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }
    }
}

fn _ui<B: Backend>(f: &mut Frame<B>) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(30),
                Constraint::Percentage(70),
            ]
            .as_ref(),
        )
        .split(f.size());

    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage( 80 ),
                Constraint::Percentage( 20 ),
            ]
            .as_ref(),
        )
        .split(chunks[1] ); 


    let block = Block::default().title("DMs").borders(Borders::ALL);
    f.render_widget(block, chunks[0]);

    let block = Block::default().title("Messages").borders(Borders::ALL);
    f.render_widget(block, right_chunks[0]);
    
    let block = Block::default().title("Enter Text Below").borders(Borders::ALL);
    f.render_widget(block, right_chunks[1]);
}


// FIXME: DELET THIS 
fn render_home<'a>() -> Paragraph<'a> {
    let home = Paragraph::new(vec![
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Welcome")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("to")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::styled(
            "pet-CLI",
            Style::default().fg(Color::LightBlue),
        )]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Press 'p' to access pets, 'a' to add random new pets and 'd' to delete the currently selected pet.")]),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Home")
            .border_type(BorderType::Plain),
    );
    home
}

// fn render_pets<'a>( dm_list_state: &ListState) -> (List<'a>, Table<'a>) {

//     let dms = Block::default()
//         .borders(Borders::ALL)
//         .style(Style::default().fg(Color::White))
//         .title("Pets")
//         .border_type(BorderType::Plain);

//     let dm_list = read_db().expect("can fetch dm list");
//     let items: Vec<_> = dm_list
//         .iter()
//         .map(|dm| {
//             ListItem::new(Spans::from(vec![Span::styled(
//                 dm.name.clone(),
//                 Style::default(),
//             )]))
//         })
//         .collect();

//     let selected_dm = dm_list
//         .get(
//             dm_list_state
//                 .selected()
//                 .expect("there is always a selected Direct Message"),
//         )
//         .expect("exists")
//         .clone();

//     let list = List::new(items).block(dms).highlight_style(
//         Style::default()
//             .bg(Color::Yellow)
//             .fg(Color::Black)
//             .add_modifier(Modifier::BOLD),
//     );

//     let dm_detail = Table::new(vec![Row::new(vec![
//         Cell::from(Span::raw(selected_dm.id.to_string())),
//         Cell::from(Span::raw(selected_dm.name)),
//         Cell::from(Span::raw(selected_dm.category)),
//         Cell::from(Span::raw(selected_dm.age.to_string())),
//         Cell::from(Span::raw(selected_dm.created_at.to_string())),
//     ])])
//     .header(Row::new(vec![
//         Cell::from(Span::styled(
//             "ID",
//             Style::default().add_modifier(Modifier::BOLD),
//         )),
//         Cell::from(Span::styled(
//             "Name",
//             Style::default().add_modifier(Modifier::BOLD),
//         )),
//         Cell::from(Span::styled(
//             "Category",
//             Style::default().add_modifier(Modifier::BOLD),
//         )),
//         Cell::from(Span::styled(
//             "Age",
//             Style::default().add_modifier(Modifier::BOLD),
//         )),
//         Cell::from(Span::styled(
//             "Created At",
//             Style::default().add_modifier(Modifier::BOLD),
//         )),
//     ]))
//     .block(
//         Block::default()
//             .borders(Borders::ALL)
//             .style(Style::default().fg(Color::White))
//             .title("Detail")
//             .border_type(BorderType::Plain),
//     )
//     .widths(&[
//         Constraint::Percentage(5),
//         Constraint::Percentage(20),
//         Constraint::Percentage(20),
//         Constraint::Percentage(5),
//         Constraint::Percentage(20),
//     ]);

//     (list, dm_detail)
// }


// fn add_random_pet_to_db() -> Result<Vec<Pet>, Error> {
//     let mut rng = rand::thread_rng();
//     let db_content = fs::read_to_string(DB_PATH)?;
//     let mut parsed: Vec<Pet> = serde_json::from_str(&db_content)?;
//     let catsdogs = match rng.gen_range(0, 1) {
//         0 => "cats",
//         _ => "dogs",
//     };

//     let random_pet = Pet {
//         id: rng.gen_range(0, 9999999),
//         name: rng.sample_iter(Alphanumeric).take(10).collect(),
//         category: catsdogs.to_owned(),
//         age: rng.gen_range(1, 15),
//         created_at: Utc::now(),
//     };

//     parsed.push(random_pet);
//     fs::write(DB_PATH, &serde_json::to_vec(&parsed)?)?;
//     Ok(parsed)
// }

// fn remove_pet_at_index(pet_list_state: &mut ListState) -> Result<(), Error> {
//     if let Some(selected) = pet_list_state.selected() {
//         let db_content = fs::read_to_string(DB_PATH)?;
//         let mut parsed: Vec<Pet> = serde_json::from_str(&db_content)?;
//         parsed.remove(selected);
//         fs::write(DB_PATH, &serde_json::to_vec(&parsed)?)?;
//         pet_list_state.select(Some(selected - 1));
//     }
//     Ok(())
// }