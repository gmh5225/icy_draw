use std::path::Path;

use eframe::{egui, epaint::Vec2};
use icy_engine::EngineResult;

use crate::{model::Tool, AnsiEditor, Commands, Message, TerminalResult};

pub trait ClipboardHandler {
    fn can_cut(&self) -> bool {
        false
    }
    fn cut(&mut self) -> EngineResult<()> {
        Ok(())
    }

    fn can_copy(&self) -> bool {
        false
    }
    fn copy(&mut self) -> EngineResult<()> {
        Ok(())
    }

    fn can_paste(&self) -> bool {
        false
    }
    fn paste(&mut self) -> EngineResult<()> {
        Ok(())
    }
}

pub trait UndoHandler {
    fn undo_description(&self) -> Option<String>;
    fn can_undo(&self) -> bool;
    /// .
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    fn undo(&mut self) -> EngineResult<Option<Message>>;

    fn redo_description(&self) -> Option<String>;
    fn can_redo(&self) -> bool;
    /// .
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    fn redo(&mut self) -> EngineResult<Option<Message>>;
}

pub trait Document: UndoHandler + ClipboardHandler {
    fn undo_stack_len(&self) -> usize;

    fn default_extension(&self) -> &'static str;

    fn get_bytes(&mut self, path: &Path) -> TerminalResult<Vec<u8>>;

    fn show_ui(&mut self, ui: &mut egui::Ui, cur_tool: &mut Box<dyn Tool>, selected_tool: usize, options: &DocumentOptions) -> Option<Message>;

    fn destroy(&self, gl: &glow::Context) -> Option<Message>;

    fn get_ansi_editor_mut(&mut self) -> Option<&mut AnsiEditor>;
    fn get_ansi_editor(&self) -> Option<&AnsiEditor>;

    fn can_paste_char(&self) -> bool {
        false
    }
    fn paste_char(&mut self, _ui: &mut eframe::egui::Ui, _ch: char) {}

    fn inform_save(&mut self) {}
}

pub struct DocumentOptions {
    scale: Vec2,
    pub commands: Commands,
    pub fit_width: bool,
}

impl DocumentOptions {
    pub fn new() -> Self {
        Self {
            scale: Vec2::new(2., 2.),
            commands: Commands::default(),
            fit_width: false,
        }
    }
    pub fn get_scale(&self) -> Vec2 {
        self.scale
    }

    pub fn set_scale(&mut self, scale: Vec2) {
        self.scale = scale.clamp(Vec2::new(0.5, 0.5), Vec2::new(5., 5.));
        self.scale = (self.scale * 100.0).floor() / 100.0;
    }
}

impl Default for DocumentOptions {
    fn default() -> Self {
        Self::new()
    }
}
