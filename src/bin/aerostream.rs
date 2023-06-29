use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;
use std::io::stdout;
use std::thread::sleep;
use std::time::Duration;

use aerostream::{Client, Commit};
use anyhow::Result;
use chrono::{DateTime, Local};
use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{
  disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use textwrap::wrap;
use tui::backend::{Backend, CrosstermBackend};
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, List, ListItem, ListState};
use tui::{Frame, Terminal};

struct Post {
  create_at: DateTime<Local>,
  did: String,
  handle: Option<String>,
  text: Vec<String>,
  blobs: Vec<String>,
}

impl From<&Commit> for Post {
  fn from(value: &Commit) -> Self {
    Self {
      create_at: value.time.with_timezone(&Local),
      did: value.repo.clone(),
      handle: None,
      text: value.get_post_text(),
      blobs: value.blobs.iter().map(|b| b.to_string()).collect(),
    }
  }
}

impl Debug for Post {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_fmt(format_args!(
      "{} : {} : {}{}",
      self.create_at,
      match &self.handle {
        Some(h) => h,
        None => &self.did,
      },
      self.text.join(" ").replace("\n", " "),
      match self.blobs.is_empty() {
        true => String::new(),
        false => format!(" : {}", self.blobs.join(",")),
      }
    ))
  }
}

impl Post {
  fn is_empty(&self) -> bool {
    self.text.iter().all(|t| t.is_empty()) && self.blobs.is_empty()
  }

  fn get_handle(&mut self, client: &mut Client) {
    self.handle = client.get_repo(&self.did).ok().map(|r| r.handle.clone());
  }

  fn get_name(&self) -> String {
    match &self.handle {
      Some(h) => h.clone(),
      None => self.did.clone(),
    }
  }

  fn to_spans(&self, width: u16) -> Vec<Spans> {
    let mut ret: Vec<Spans> = Vec::new();
    let name = self.get_name();
    if name.len() > width as usize - 8 {
      ret.push(Spans::from(Span::styled(
        format!("{} {}", self.create_at.format("%H:%M"), name),
        Style::default().fg(Color::White).bg(Color::Blue),
      )));
    } else {
      ret.push(Spans::from(Span::styled(
        format!(
          "{} {}{}",
          self.create_at.format("%H:%M"),
          name,
          " ".repeat(width as usize - name.len() - 8)
        ),
        Style::default().fg(Color::White).bg(Color::Blue),
      )));
    }
    for text in self.text.iter() {
      for line in wrap(text, (width - 3) as usize).iter() {
        ret.push(Spans::from(Span::styled(
          line.to_string(),
          Style::default(),
        )));
      }
    }
    if !self.blobs.is_empty() {
      ret.push(Spans::from(Span::styled(
        "🖼️".repeat(self.blobs.len()),
        Style::default(),
      )));
    }
    ret
  }
}

fn ui<B: Backend>(
  f: &mut Frame<B>,
  filters: &mut HashMap<String, (VecDeque<Post>, ListState)>,
  focus: usize,
) {
  let count = filters.len();
  let columns = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([Constraint::Ratio(1, count as u32)].repeat(count))
    .split(f.size());
  let mut filter_names = filters.keys().cloned().collect::<Vec<_>>();
  filter_names.sort();
  for (index, filter) in filter_names.into_iter().enumerate() {
    if let Some((events, ref mut state)) = filters.get_mut(&filter) {
      let column = columns[index];
      f.render_stateful_widget(
        List::new(
          events
            .iter()
            .map(|post| ListItem::new(post.to_spans(column.width)))
            .collect::<Vec<_>>(),
        )
        .block(
          Block::default()
            .borders(Borders::ALL)
            .border_style(match focus == index {
              true => Style::default().fg(Color::White),
              false => Style::default().fg(Color::DarkGray),
            })
            .title(filter.as_str()),
        )
        .highlight_style(Style::default().fg(Color::Black).bg(Color::White)),
        column,
        state,
      );
    };
  }
}

fn main() -> Result<()> {
  env_logger::init();
  let max_len: usize = std::env::var("MAX_LEN")
    .ok()
    .and_then(|s| s.parse().ok())
    .unwrap_or(100);

  enable_raw_mode()?;
  let mut stdout = stdout();
  execute!(stdout, EnterAlternateScreen)?;
  let backend = CrosstermBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;
  let mut client = Client::default();

  client.set_timeout(5);
  client.connect_ws()?;

  let mut filters = client
    .get_filter_names()
    .into_iter()
    .map(|f| (f, (VecDeque::new(), ListState::default())))
    .collect::<HashMap<String, (VecDeque<Post>, ListState)>>();
  let mut focus = 0;
  if filters.is_empty() {
    filters.insert(String::from(""), (VecDeque::new(), ListState::default()));
  }
  terminal.draw(|f| ui(f, &mut filters, focus))?;
  loop {
    let mut updated = false;
    for (filter, event) in client.next_event_filtered_all()?.into_iter() {
      if !event.is_empty() {
        if let Some((posts, state)) = filters.get_mut(&filter) {
          if let Some(commit) = event.as_commit() {
            let mut post = Post::from(commit);
            if !post.is_empty() {
              post.get_handle(&mut client);
              posts.push_front(post);
              posts.truncate(max_len);
              if let Some(s) = state.selected() {
                state.select(Some(s + 1));
              }
              updated = true;
            }
          }
        }
      }
    }
    if crossterm::event::poll(Duration::from_millis(10))? {
      match crossterm::event::read()? {
        Event::Key(key) => match &key.kind {
          KeyEventKind::Press => match &key.code {
            KeyCode::Char('q') => break,
            KeyCode::Char('c') => {
              if key.modifiers.contains(KeyModifiers::CONTROL) {
                break;
              }
            }
            KeyCode::Char('r') => {
              if key.modifiers.contains(KeyModifiers::CONTROL) {
                terminal.clear()?;
                updated = true;
              }
            }
            KeyCode::F(5) => {
              terminal.clear()?;
              updated = true;
            }
            KeyCode::Char('s') => {
              let mut filter_names = filters.keys().cloned().collect::<Vec<_>>();
              filter_names.sort();
              if let Some((posts, state)) = filters.get(&filter_names[focus]) {
                if let Some(s) = state.selected() {
                  if let Some(p) = posts.get(s) {
                    if let Some(handle) = &p.handle {
                      client.subscribe_handle("Favorites", handle)?;
                    }
                  }
                }
              }
            }
            KeyCode::Char('u') => {
              let mut filter_names = filters.keys().cloned().collect::<Vec<_>>();
              filter_names.sort();
              if let Some((posts, state)) = filters.get(&filter_names[focus]) {
                if let Some(s) = state.selected() {
                  if let Some(p) = posts.get(s) {
                    if let Some(handle) = &p.handle {
                      client.unsubscribe_handle("Favorites", handle)?;
                    }
                  }
                }
              }
            }
            KeyCode::Right => {
              let mut filter_names = filters.keys().cloned().collect::<Vec<_>>();
              filter_names.sort();
              if let Some((_, state)) = filters.get_mut(&filter_names[focus]) {
                state.select(None);
              }
              match focus + 1 >= filters.len() {
                true => focus = 0,
                false => focus += 1,
              }
              updated = true;
            }
            KeyCode::Left => {
              let mut filter_names = filters.keys().cloned().collect::<Vec<_>>();
              filter_names.sort();
              if let Some((_, state)) = filters.get_mut(&filter_names[focus]) {
                state.select(None);
              }
              match focus == 0 {
                true => focus = filters.len() - 1,
                false => focus -= 1,
              }
              updated = true
            }
            KeyCode::Up => {
              let mut filter_names = filters.keys().cloned().collect::<Vec<_>>();
              filter_names.sort();
              if let Some((posts, state)) = filters.get_mut(&filter_names[focus]) {
                if let Some(s) = state.selected() {
                  if s == 0 {
                    state.select(None);
                  } else {
                    state.select(Some(s - 1));
                  }
                } else {
                  if !posts.is_empty() {
                    state.select(Some(posts.len() - 1));
                  }
                }
                updated = true;
              }
            }
            KeyCode::Down => {
              let mut filter_names = filters.keys().cloned().collect::<Vec<_>>();
              filter_names.sort();
              if let Some((posts, state)) = filters.get_mut(&filter_names[focus]) {
                if let Some(s) = state.selected() {
                  if s >= posts.len() - 1 {
                    state.select(None);
                  } else {
                    state.select(Some(s + 1));
                  }
                } else {
                  if !posts.is_empty() {
                    state.select(Some(0));
                  }
                }
                updated = true;
              }
            }
            KeyCode::Esc => {
              for (_, (_, state)) in filters.iter_mut() {
                state.select(None);
              }
              updated = true;
            }
            _ => (),
          },
          _ => (),
        },
        Event::Resize(_, _) => updated = true,
        _ => (),
      }
    }
    if updated {
      terminal.draw(|f| ui(f, &mut filters, focus))?;
    } else {
      sleep(Duration::from_millis(10));
    }
  }
  disable_raw_mode()?;
  execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
  terminal.show_cursor()?;
  Ok(())
}
