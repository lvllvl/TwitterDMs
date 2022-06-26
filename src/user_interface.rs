// use mini_redis::{ client, Result };
// use crossterm::{
//     event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
//     execute,
//     terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
// };
// use futures::executor::block_on; 

// use std::{error::Error, io};
// use tui::{
//     backend::{Backend, CrosstermBackend},
//     layout::{Constraint, Direction, Layout},
//     widgets::{Block, Borders},
//     Frame, Terminal,
// };

// mod user_interace {
    
//     fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
//         loop {
//             terminal.draw(|f| ui(f))?;

//             if let Event::Key(key) = event::read()? {
//                 if let KeyCode::Char('q') = key.code {
//                     return Ok(());
//                 }
//             }
//         }
//     }

//     fn ui<B: Backend>(f: &mut Frame<B>) {
//         let chunks = Layout::default()
//             .direction(Direction::Horizontal)
//             .constraints(
//                 [
//                     Constraint::Percentage(30),
//                     Constraint::Percentage(70),
//                 ]
//                 .as_ref(),
//             )
//             .split(f.size());

//         let right_chunks = Layout::default()
//             .direction(Direction::Vertical)
//             .constraints(
//                 [
//                     Constraint::Percentage( 80 ),
//                     Constraint::Percentage( 20 ),
//                 ]
//                 .as_ref(),
//             )
//             .split(chunks[1] ); 


//         let block = Block::default().title("DMs").borders(Borders::ALL);
//         f.render_widget(block, chunks[0]);

//         let block = Block::default().title("Messages").borders(Borders::ALL);
//         f.render_widget(block, right_chunks[0]);
        
//         let block = Block::default().title("Enter Text Below").borders(Borders::ALL);
//         f.render_widget(block, right_chunks[1]);
//     }

// }