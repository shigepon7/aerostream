use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;
use std::io::stdout;
use std::thread::sleep;
use std::time::Duration;

use aerostream::api::ComAtprotoSyncSubscribereposCommit;
use aerostream::Client;
use anyhow::Result;
use chrono::{DateTime, Local};
use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{
  disable_raw_mode, enable_raw_mode, window_size, EnterAlternateScreen, LeaveAlternateScreen,
};
use image::{load_from_memory, DynamicImage};
use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState};
use ratatui::{Frame, Terminal};
use ratatui_image::picker::Picker;
use ratatui_image::{FixedImage, Resize};
use textwrap::wrap;

struct Post {
  create_at: DateTime<Local>,
  did: String,
  handle: Option<String>,
  text: Vec<String>,
  blobs: Vec<String>,
  path: Option<String>,
}

impl From<&ComAtprotoSyncSubscribereposCommit> for Post {
  fn from(value: &ComAtprotoSyncSubscribereposCommit) -> Self {
    Self {
      create_at: value.time.with_timezone(&Local),
      did: value.repo.clone(),
      handle: None,
      text: value.get_post_text(),
      blobs: value.blobs.iter().map(|b| b.to_string()).collect(),
      path: value.get_post_path(),
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

  fn to_spans(&self, width: u16) -> Vec<Line> {
    let mut ret: Vec<Line> = Vec::new();
    let name = self.get_name();
    if name.len() > width as usize - 8 {
      ret.push(Line::from(Span::styled(
        format!("{} {}", self.create_at.format("%H:%M"), name),
        Style::default().fg(Color::White).bg(Color::Blue),
      )));
    } else {
      ret.push(Line::from(Span::styled(
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
        ret.push(Line::from(Span::styled(line.to_string(), Style::default())));
      }
    }
    if !self.blobs.is_empty() {
      ret.push(Line::from(Span::styled(
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
  image_status: &mut (Option<String>, Option<DynamicImage>),
) {
  if let Some(img) = &image_status.0 {
    if let Ok(res) = ureq::get(&img).call() {
      let mut buf: Vec<u8> = Vec::new();
      if res.into_reader().read_to_end(&mut buf).is_ok() {
        if let Ok(img_data) = load_from_memory(buf.as_slice()) {
          *image_status = (None, Some(img_data));
        }
      }
    }
  }
  if let Some(img_data) = &image_status.1 {
    let mut size = f.size().clone();
    if img_data.width() * (2 * f.size().height as u32) > img_data.height() * (f.size().width as u32)
    {
      size.height = (((f.size().width as u32) * img_data.height() / img_data.width()) as u16) / 2;
    } else {
      size.width = ((2 * f.size().height as u32) * img_data.width() / img_data.height()) as u16;
    };
    let font_size = match window_size() {
      Ok(wsize) => (wsize.width / wsize.columns, wsize.height / wsize.rows),
      Err(_) => (5, 10),
    };
    let mut picker = Picker::new(font_size);
    f.render_widget(
      FixedImage::new(
        picker
          .new_protocol(img_data.clone(), size, Resize::Fit)
          .unwrap()
          .as_ref(),
      ),
      size,
    );
    return;
  }
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
  // fern::Dispatch::new()
  //   .format(|out, message, record| {
  //     out.finish(format_args!(
  //       "[{} {}] {}",
  //       record.level(),
  //       record.target(),
  //       message
  //     ))
  //   })
  //   .level(log::LevelFilter::Debug)
  //   .chain(fern::log_file("output.log")?)
  //   .apply()?;
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

  if let Ok(id) = std::env::var("BSKY_ID") {
    if let Ok(pw) = std::env::var("BSKY_PW") {
      client.login(&id, &pw)?;
      client.add_timeline(&id)?;
    }
  }

  client.set_timeout(5);
  client.connect_ws()?;

  let mut filters = client
    .get_filter_names()
    .into_iter()
    .map(|f| (f, (VecDeque::new(), ListState::default())))
    .collect::<HashMap<String, (VecDeque<Post>, ListState)>>();
  let mut focus = 0;
  let mut image_status = (None, None);
  let mut image_index = 0;
  if filters.is_empty() {
    filters.insert(String::from(""), (VecDeque::new(), ListState::default()));
  }
  terminal.draw(|f| ui(f, &mut filters, focus, &mut image_status))?;
  loop {
    let mut updated = false;
    for (filter, event) in client.next_event_filtered_all()?.into_iter() {
      if let Some((posts, state)) = filters.get_mut(&filter) {
        if let Some(commit) = event.as_commit() {
          log::debug!("--- COMMIT {:?}", commit);
          let mut post = Post::from(commit);
          log::debug!("xxx POST {:?}", post);
          if !post.is_empty() {
            log::debug!("||| POST {:?}", post);
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
            KeyCode::Char('i') => {
              let mut filter_names = filters.keys().cloned().collect::<Vec<_>>();
              filter_names.sort();
              if let Some((posts, state)) = filters.get(&filter_names[focus]) {
                if let Some(sel) = state.selected() {
                  if let Some(p) = posts.get(sel) {
                    image_status = (p.blobs.first().cloned(), None);
                    image_index = 0;
                    updated = true;
                  }
                }
              }
            }
            KeyCode::Char('j') => {
              let mut filter_names = filters.keys().cloned().collect::<Vec<_>>();
              filter_names.sort();
              if let Some((posts, state)) = filters.get_mut(&filter_names[focus]) {
                if let Some(mut sel) = state.selected() {
                  if let Some(p) = posts.get(sel) {
                    image_index += 1;
                    updated = true;
                    if image_index >= p.blobs.len() {
                      sel += 1;
                      image_index = 0;
                    } else {
                      image_status = (p.blobs.get(image_index).cloned(), None);
                    }
                  }
                  if image_index == 0 {
                    while sel < posts.len() - 1 {
                      if let Some(p) = posts.get(sel) {
                        if !p.blobs.is_empty() {
                          state.select(Some(sel));
                          image_status = (p.blobs.first().cloned(), None);
                          break;
                        }
                      }
                      sel += 1;
                    }
                    if sel >= posts.len() - 1 {
                      image_status = (None, None);
                    }
                  }
                }
              }
            }
            KeyCode::Char('k') => {
              let mut filter_names = filters.keys().cloned().collect::<Vec<_>>();
              filter_names.sort();
              if let Some((posts, state)) = filters.get_mut(&filter_names[focus]) {
                if let Some(mut sel) = state.selected() {
                  if let Some(p) = posts.get(sel) {
                    updated = true;
                    if image_index > 0 {
                      image_index -= 1;
                      image_status = (p.blobs.get(image_index).cloned(), None);
                    } else {
                      if sel > 0 {
                        sel -= 1;
                      }
                      image_index = usize::MAX;
                    }
                  }
                  if image_index == usize::MAX {
                    while sel > 0 {
                      if let Some(p) = posts.get(sel) {
                        if !p.blobs.is_empty() {
                          state.select(Some(sel));
                          image_status = (p.blobs.first().cloned(), None);
                          break;
                        }
                      }
                      sel -= 1;
                    }
                    if sel == 0 {
                      image_status = (None, None);
                    }
                  }
                }
              }
            }
            KeyCode::Enter => {
              let mut filter_names = filters.keys().cloned().collect::<Vec<_>>();
              filter_names.sort();
              if let Some((posts, state)) = filters.get(&filter_names[focus]) {
                if let Some(sel) = state.selected() {
                  if let Some(p) = posts.get(sel) {
                    if image_status.1.is_some() {
                      if let Some(img) = p.blobs.get(image_index) {
                        webbrowser::open(img).ok();
                      }
                    } else {
                      log::warn!("{:?}", p);
                      if let Some(handle) = &p.handle {
                        if let Some(path) = &p.path {
                          if let Some(ts) = path.split("/").nth(1) {
                            let url = format!("https://bsky.app/profile/{}/post/{}", handle, ts);
                            log::warn!("{}", url);
                            webbrowser::open(&url).ok();
                          }
                        }
                      }
                    }
                  }
                }
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
              image_status = (None, None);
              updated = true;
            }
            KeyCode::Backspace => {
              image_status = (None, None);
              updated = true;
            }
            KeyCode::Char('a') => {
              let mut filter_names = filters.keys().cloned().collect::<Vec<_>>();
              filter_names.sort();
              if let Some((posts, state)) = filters.get(&filter_names[focus]) {
                if let Some(s) = state.selected() {
                  if let Some(p) = posts.get(s) {
                    if let Some(handle) = &p.handle {
                      client.add_timeline(handle)?;
                    }
                  }
                }
              }
            }
            KeyCode::Char('d') => {
              let mut filter_names = filters.keys().cloned().collect::<Vec<_>>();
              filter_names.sort();
              if let Some((posts, state)) = filters.get(&filter_names[focus]) {
                if let Some(s) = state.selected() {
                  if let Some(p) = posts.get(s) {
                    if let Some(handle) = &p.handle {
                      client.remove_timeline(handle);
                    }
                  }
                }
              }
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
      terminal.draw(|f| ui(f, &mut filters, focus, &mut image_status))?;
    } else {
      sleep(Duration::from_millis(10));
    }
  }
  disable_raw_mode()?;
  execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
  terminal.show_cursor()?;
  Ok(())
}
