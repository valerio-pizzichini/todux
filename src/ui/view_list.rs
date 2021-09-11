use crate::todo;
use crate::ui;
use crate::database;

use termion::{event::Key, raw::IntoRawMode, input::TermRead};
use tui::widgets::{Block, Borders, List, ListItem};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use std::io::{self, Read};
use todo::{TodoData};
use std::error::Error;
use tui::Terminal;
use tui::backend::TermionBackend;
use ui::todolist::TodoList;

pub fn show_list(
    mut todo_list: TodoList,
    db_filename: String
) -> Result<(), Box<dyn Error>> {

    let stdout = io::stdout()
        .into_raw_mode()
        .expect("asd");
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).expect("asd");
    let mut asi = termion::async_stdin();
    terminal.clear().expect("Error clearing terminal");

    loop {
        terminal.draw(|f| {
            // Iterate through all elements in the `items` app and append some debug text to it.
            let items: Vec<ListItem> = todo_list
                .items
                .items
                .iter()
                .map(|i| {
                    let content = Span::from(String::from(i.title.as_str()));
                    let checkbox = match i.done {
                        true => Span::from("[x] "),
                        false => Span::from("[ ] ")
                    };
                    let spans = Spans::from(vec![checkbox, content]);
                    ListItem::new(spans).style(Style::default().fg(Color::Black).bg(Color::White))
                })
                .collect();

            // Create a List from all list items and highlight the currently selected one
            let items = List::new(items)
                .block(Block::default().borders(Borders::ALL).title("TODO"))
                .highlight_style(
                    Style::default()
                        .bg(Color::LightGreen)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(">> ");

            f.render_stateful_widget(items, f.size(), &mut todo_list.items.state);
        })?;

        for k in asi.by_ref().keys() {
            match k.unwrap() {
                Key::Char('q') => {
                    terminal.clear()?;
                    // Save current todo list before exit
                    database::save(&TodoData {
                        todos: todo_list.items.items
                    }, &db_filename);
                    return Ok(());
                },
                Key::Up => {
                    todo_list.items.previous()
                },
                Key::Down => {
                    todo_list.items.next()
                },
                Key::Char('t') => {
                    &todo_list.items.items[todo_list.items.state.selected().unwrap()].toggle();
                },
                Key::Char('d') => {
                    todo_list.items.remove()
                }
                _ => (),
            }
        }
    }
}
