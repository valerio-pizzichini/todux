use crate::todo;
use crate::ui;
use crate::database;
use crate::key_binding;
use crate::log;
use crate::command;

use termion::{event::Key, raw::IntoRawMode, input::TermRead};
use tui::widgets::{Block, Borders, List, ListItem, Paragraph};
use tui::style::{Color, Modifier, Style};
use tui::layout::{Layout, Constraint, Direction};
use tui::text::{Span, Spans, Text};
use std::io::{self, Read};
use todo::{TodoData};
use std::error::Error;
use tui::Terminal;
use tui::backend::TermionBackend;
use ui::todolist::TodoList;

pub fn show_list(
    mut todo_list: TodoList,
    db_filename: String,
    workspace_name: String
) -> Result<(), Box<dyn Error>> {

    let stdout = io::stdout()
        .into_raw_mode()
        .expect("Unable to listen input on stdout");
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).expect("Unable to select terminal");
    let mut asi = termion::async_stdin();

    // Clear current terminal before drawing the app to avoid conflicts
    terminal
        .clear()
        .expect("Error clearing terminal");

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Max(99),
                        Constraint::Max(1),
                    ]
                    .as_ref(),
                )
                .split(f.size());

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

            // Create a List from all list items and highlight the selected one
            let items = List::new(items)
                .block(Block::default().borders(Borders::ALL).title("TODO"))
                .highlight_style(
                    Style::default()
                        .bg(Color::LightGreen)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(">> ");
            
            f.render_stateful_widget(items, chunks[0], &mut todo_list.items.state);
            
            let workspace_text = Spans::from(
                Span::styled(
                    format!("Workspace: {}{}", workspace_name, get_key_bindings_string()), 
                    Style::default().add_modifier(Modifier::ITALIC).add_modifier(Modifier::BOLD)
                ),
            );
            let workspace_bar = Paragraph::new(workspace_text)
                .block(
                    Block::default()
                        .style(
                            Style::default()
                                .bg(Color::White)
                                .fg(Color::Black)
                                .add_modifier(Modifier::BOLD)
                        ),
                );

            f.render_widget(workspace_bar, chunks[1]);
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
                },
                _ => (),
            }
        }
    }
}

fn get_key_bindings_string() -> String {
    let mut key_bindings_string = String::from("");

    key_binding::get_key_bindings()
        .into_iter()
        .for_each(|f| {
            let key_binding_string = format!("   [{}] {}", f.key, f.description);
            key_bindings_string.push_str(&key_binding_string);
        });

    key_bindings_string
}
