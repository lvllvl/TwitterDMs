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
        Block, Borders, BorderType, Cell, List, ListItem, ListState, Paragraph, Row, Table, Tabs, Wrap,
    },
    text::{ Span, Spans },
    Frame, Terminal,
};

mod users; 
mod config; 
mod twitter_api; 
mod direct_messages;

// #[derive(Serialize, Deserialize, Clone)]
// struct Pet {
//     id: usize,
//     name: String,
//     category: String,
//     age: usize,
//     created_at: DateTime<Utc>,
// }

// Data structure for input events
enum Event<I> {
    Input(I),
    Tick,
}
#[derive( Copy, Clone, Debug ) ]
enum MenuItem {
    Home,
    DMs,
    // EnterTextMessage,
    // Tweeting,
}
impl From<MenuItem> for usize {
    fn from( input: MenuItem ) -> usize {
        match input {
            MenuItem::Home => 0,
            MenuItem::DMs => 1, 
            // MenuItem::EnterTextMessage => 2, 
            // MenuItem::Tweeting => 3, 
        }
    }
}

#[ tokio::main ]
async fn main() -> Result<()> { // FIXME: async / return values are different 
// async fn main() -> Result<(), Box<dyn Error>> { // FIXME: async / return values are different 

    let connection = rusqlite::Connection::open("direct_messages.db")?; // Create a SQLite table
    let mut friends_list = HashMap::new();
    let mut message_list: HashMap<u64, u64> = HashMap::new();
    message_list.insert( 123456, 123456 ); // <k, v> == message_id, count_num, keep track of all messages recieved 
    
    // Get authorization, then organize messages   
    let request_auth = twitter_api::requesting_user_authorization( &connection ); 
    let request_auth: std::result::Result< users::Users, Box<dyn std::error::Error>> = request_auth.await;
    let request_auth = request_auth.unwrap(); // Get token, user_id, screen_name, sqlite connection

    friends_list.insert( request_auth.user_id.clone(), request_auth.screen_name.clone() ); 

    // let _messages = twitter_api::get_direct_messages( &request_auth, &connection, &mut message_list ).await;  // Get all messages
    // let _dms_list = read_db( &connection );  // Get all messages from database

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
    // twitter_api::_send_dm(String::from( "1:41, Monday another test!" ), request_auth.user_id, &request_auth, &connection ).await.unwrap(); // Send a DM
    // twitter_api::update_direct_messages( &request_auth, &connection, &mut message_list).await.unwrap(); 

    let (tx, rx) = mpsc::channel();
    //let tick_rate = Duration::from_millis(200);
    let tick_rate = Duration::from_millis(900);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                if let CEvent::Key(key) = event::read().expect("can read events") {
                    tx.send(Event::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let menu_titles = vec!["Home", "DMs", "Quit"]; 
    let mut active_menu_item = MenuItem::Home; // TODO: update based on your Menu Item 
    let mut dm_list_state = ListState::default(); 
    dm_list_state.select(Some(0));

    loop {
        terminal.draw(|rect| {
            let size = rect.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Min(2),
                        Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .split(size);

            let copyright = Paragraph::new("TwitterDMs-CLI 2022.")
                .style(Style::default().fg(Color::LightCyan))
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::White))
                        .title("Copyright")
                        .border_type(BorderType::Plain),
                );

            let menu = menu_titles
                .iter()
                .map(|t| {
                    let (first, rest) = t.split_at(1);
                    Spans::from(vec![
                        Span::styled(
                            first,
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::UNDERLINED),
                        ),
                        Span::styled(rest, Style::default().fg(Color::White)),
                    ])
                })
                .collect();

            let tabs = Tabs::new(menu)
                .select(active_menu_item.into())
                .block(Block::default().title("Menu").borders(Borders::ALL))
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().fg(Color::Yellow))
                .divider(Span::raw("|"));

            rect.render_widget(tabs, chunks[0]);
            match active_menu_item {
                MenuItem::Home => rect.render_widget(render_home(), chunks[1]),
                MenuItem::DMs => {
                    let pets_chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints(
                            [Constraint::Percentage(15), Constraint::Percentage(85)].as_ref(),
                        )
                        .split(chunks[1]);
                    let (left, right) = render_pets( &dm_list_state, &connection, &request_auth );
                    rect.render_stateful_widget(left, pets_chunks[0], &mut dm_list_state);
                    rect.render_widget(right, pets_chunks[1]);
                },
                // MenuItem::EnterTextMessage=> rect.render_widget(render_home(), chunks[1]),
                // MenuItem::Tweeting=> rect.render_widget(render_home(), chunks[1]),
            }
            rect.render_widget(copyright, chunks[2]);
        })?;

        match rx.recv()? {
        Event::Input(event) => match event.code {
            KeyCode::Char('q') => {
                disable_raw_mode()?;
                terminal.show_cursor()?;
                break;
            }
            KeyCode::Char('h') => active_menu_item = MenuItem::Home,
            KeyCode::Char('p') => active_menu_item = MenuItem::DMs,
            // KeyCode::Char('a') => {
            //     // add_random_pet_to_db().expect("can add new random pet");
            //     println!("Figure out what to do with this !")
            // }
            // KeyCode::Char('d') => {
            //     // remove_pet_at_index(&mut dm_list_state).expect("can remove pet");
            //     println!("Figure out what to do with this !")
            // }
            KeyCode::Down => {
                if let Some(selected) = dm_list_state.selected() {
                    let amount_pets = read_db( &connection ).expect("can fetch dm list").len();
                    if selected >= amount_pets - 1 {
                        dm_list_state.select(Some(0));
                    } else {
                        dm_list_state.select(Some(selected + 1));
                    }
                }
            }
            KeyCode::Up => {
                if let Some(selected) = dm_list_state.selected() {
                    let amount_pets = read_db( &connection ).expect("can fetch dm list").len();
                    if selected > 0 {
                        dm_list_state.select(Some(selected - 1));
                    } else {
                        dm_list_state.select(Some(amount_pets - 1));
                    }
                }
            }
            _ => {}
        },
        Event::Tick => {}
    } 
    }
    Ok(())
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
            "TwitterDMs-CLI",
            Style::default().fg(Color::LightBlue),
        )]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Press 'p' to access direct messages, 'a' to ______ and 'd' to _______.")]),
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

/// Pull message information from the database 
/// Deliver a HashMap with:
/// Keys = conversation_IDs
/// Values = Vec[ Messages ] 
fn read_db( conn: &Connection ) -> Result<HashMap<u64, Vec< direct_messages::Messages>>> {
    
    // FIXME: You don't need to get all this info, collect only necessary info for Message struct
    //let mut stmt = conn.prepare( "SELECT * FROM direct_messages ORDER BY convo_id, created_at ASC")?;
    let mut stmt = conn.prepare( "SELECT * FROM direct_messages ORDER BY convo_id, created_at ASC")?;
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

    let mut convos_dict = HashMap::new();

    for message in message_iter {
        let msg = message.unwrap();
        let key = msg.conversation_id.clone(); 
        convos_dict.entry( key ).or_insert( Vec::new() ).push( msg ); // FIXME: insert at index 0, instead of push
    }
    Ok( convos_dict ) 
}


/// Prepare the direct messages to be displayed
fn render_pets<'a>( dm_list_state: &ListState, connection: &Connection, user_token: &users::Users ) -> (List<'a>, Table<'a>) {

    let dms = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title("DMs")
        .border_type(BorderType::Plain);

    let dm_list = read_db( connection ).expect("can fetch dm list");
    // let ans = hash_ans.keys().cloned().collect::<Vec<String>>();
    let mut ordered_list = dm_list.keys().cloned().collect::<Vec<u64>>();
    ordered_list.sort();

    // Get dm_list keys into a list of screen names
    // let items: Vec<_> = dm_list
    //     .iter()
    //     .map( |dm| {
    //         ListItem::new( Spans::from( vec![ Span::styled(
    //             // dm.0.to_string(), 
    //             twitter_api::get_sn_by_convo_id( user_token, connection, dm.0.clone() ).unwrap(), 
    //             Style::default() 
    //         )]))
    // }).collect();
    
    let items: Vec<_> = ordered_list 
        .iter()
        .map( |dm| {
            ListItem::new( Spans::from( vec![ Span::styled(
                // dm.0.to_string(), 
                twitter_api::get_sn_by_convo_id( user_token, connection, dm.clone() ).unwrap(), 
                Style::default() 
            )]))
    }).collect();

    let mut keys = dm_list.keys().cloned().collect::<Vec< u64 >>();
    keys.sort();

    let single_key = keys.get( dm_list_state.selected().expect( "something is always selcted"))
                    .expect( "Exists" )
                    .clone();
    let selected_dms = dm_list.get( &single_key ).expect( "There is always a dm selected.");

    let list = List::new(items).block(dms).highlight_style(
        Style::default()
            .bg(Color::LightRed)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );
    
    let mut rows_vec = Vec::new();
    for s_dms in selected_dms {

       // FIXME: Figure out how to wrap text 
       rows_vec.push(  // Blank row 
        Row::new( vec![
        Cell::from( Span::raw( String::from( "  " ))),
        Cell::from( Span::raw( String::from( "  " ))),
        Cell::from( Span::raw( String::from( "  " ))),
    ])
    );

       rows_vec.push( 
        Row::new(vec![ // actual content for each row 
            Cell::from(Span::raw( s_dms.sender_screen_name.clone() ) ),
            Cell::from(Span::raw( s_dms.created_at.to_string() ) ),
            Cell::from(Span::raw( s_dms.text.to_string() ) ),
            ])) ;
    }

    let dm_detail = Table::new(
        rows_vec 
)
    .header(Row::new(vec![
        Cell::from(Span::styled(
            "name",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "sent @",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "message",
            Style::default().add_modifier(Modifier::BOLD),
        )),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Detail")
            .border_type(BorderType::Plain),
    )
    .widths(&[
        Constraint::Percentage(10),
        Constraint::Percentage(10),
        Constraint::Percentage(80),
    ]);

    ( list, dm_detail )
} 
