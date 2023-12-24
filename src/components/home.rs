use std::{collections::HashMap, time::Duration};

use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::UnboundedSender;
use super::{Component, Frame};
use crate::{
  action::Action,
  config::{Config, KeyBindings},
};

#[derive(Default)]
pub struct Home {
  command_tx: Option<UnboundedSender<Action>>,
  config: Config,
}

impl Home {
  pub fn new() -> Self {
    Self::default()
  }
}

impl Component for Home {
  fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
    self.command_tx = Some(tx);
    Ok(())
  }

  fn register_config_handler(&mut self, config: Config) -> Result<()> {
    self.config = config;
    Ok(())
  }

  fn update(&mut self, action: Action) -> Result<Option<Action>> {
    match action {
      Action::Tick => {
      },
      _ => {},
    }
    Ok(None)
  }

  fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
    let layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints(vec![
        Constraint::Percentage(23),
        Constraint::Percentage(20),
    ]
    )
    
    .split(f.size());
    let title= " 
     █████  ██   ██ ██   ██ ███████  █████  ██   ██  ██████  ██    ██ ███████      █████  ██████   ██████  █████  ██████  ███████     
    ██   ██ ██  ██  ██   ██ ██      ██   ██ ██  ██  ██    ██ ██    ██ ██          ██   ██ ██   ██ ██      ██   ██ ██   ██ ██          
    ███████ █████   ███████ ███████ ███████ █████   ██    ██ ██    ██ ███████     ███████ ██████  ██      ███████ ██   ██ █████       
    ██   ██ ██  ██  ██   ██      ██ ██   ██ ██  ██  ██    ██  ██  ██       ██     ██   ██ ██   ██ ██      ██   ██ ██   ██ ██          
    ██   ██ ██   ██ ██   ██ ███████ ██   ██ ██   ██  ██████    ████   ███████     ██   ██ ██   ██  ██████ ██   ██ ██████  ███████     
    ";
    let title= Paragraph::new(title).light_green().alignment(Alignment::Center).block(
      Block::default().borders(Borders::ALL)
    );
      f.render_widget(title, layout[0]);
      f.render_widget( Paragraph::new("press <Enter> to start").style(Style::default().light_green()).alignment(Alignment::Center).italic(), layout[1]);
    Ok(())
  }
}

