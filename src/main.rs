use mini_redis::{ client, Result };
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use futures::executor::block_on; 

use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders},
    Frame, Terminal,
};

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

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let res = run_app(&mut terminal);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f))?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>) {
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
