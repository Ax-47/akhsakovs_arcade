use std::{collections::HashMap, time::Duration};

use super::{Component, Frame};
use crate::{
    action::Action,
    config::{Config, KeyBindings},
};
use color_eyre::{eyre::Result, owo_colors::OwoColorize};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::UnboundedSender;


#[derive( Default, Clone)]
pub struct GamesArchive<'a> {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    selection: usize,
    max_selection: usize,
    items_selection: Vec<Line<'a>>,
    cache : usize,
}

impl GamesArchive<'_> {
    pub fn new() -> Self {
        Self::default().create()
    }
    pub fn create(&mut self)->Self{
        let style =Style::default().light_green();
        self.items_selection=vec![Line::styled("tetris",style), Line::styled("snake_eats_apples",style)];
        
        self.max_selection = self.items_selection.iter().len() - 1;
         self.to_owned()
        
    }
}

impl Component for GamesArchive<'_> {
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
            Action::Tick => {}
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(23),
                Constraint::Percentage(100 - 23),
            ])
            .split(f.size());
        let title= " 
  ██████   █████  ███    ███ ███████ ███████      █████  ██████   ██████ ██   ██ ██ ██    ██ ███████ 
 ██       ██   ██ ████  ████ ██      ██          ██   ██ ██   ██ ██      ██   ██ ██ ██    ██ ██      
 ██   ███ ███████ ██ ████ ██ █████   ███████     ███████ ██████  ██      ███████ ██ ██    ██ █████   
 ██    ██ ██   ██ ██  ██  ██ ██           ██     ██   ██ ██   ██ ██      ██   ██ ██  ██  ██  ██      
  ██████  ██   ██ ██      ██ ███████ ███████     ██   ██ ██   ██  ██████ ██   ██ ██   ████   ███████ 
    ";
        let title = Paragraph::new(title)
            .light_green()
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(title, layout[0]);
        
        let scrollbar = Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .thumb_style(Style::default().light_green())
            .begin_style(Style::default().light_green())
            .end_style(Style::default().light_green())
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"));
        
       
        let paragraph = Paragraph::new(self.items_selection.clone())
            .alignment(Alignment::Center)
            .block(Block::new().borders(Borders::RIGHT));
        let mut scrollbar_state = ScrollbarState::new(self.items_selection.iter().len()).position(self.selection);
        f.render_widget(paragraph, layout[1]);
        f.render_stateful_widget(
            scrollbar,
            layout[1], // using a inner vertical margin of 1 unit makes the scrollbar inside the block
            &mut scrollbar_state,
        );
        Ok(())
    }
    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        let selection_style=Style::default().black().on_light_green();
        let unselection_style=Style::default().reset().light_green();
        self.cache =self.selection;
        match key.code {
            KeyCode::Up => {
                if self.selection == self.max_selection {
                    self.selection = 0;
                } else {
                    self.selection += 1;
                }
                self.items_selection[self.selection].patch_style(selection_style);
                self.items_selection[self.cache].patch_style(unselection_style);
            }
            KeyCode::Down => {
                if self.selection == 0 {
                    self.selection = self.max_selection;
                } else {
                    self.selection -= 1;
                }
                self.items_selection[self.selection].patch_style(selection_style);
                self.items_selection[self.cache].patch_style(unselection_style);
            }
            _ => {}
        }
        Ok(None)
    }
}
