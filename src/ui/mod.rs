mod main_window;
use std::error::Error;

use eframe::egui;
pub use main_window::*;

mod document;
pub use document::*;

mod palette_editor;
pub use palette_editor::*;

mod tool_switcher;
pub use tool_switcher::*;


mod icons;
pub use icons::*;

mod settings;
pub use settings::*;

mod dialogs;
pub use dialogs::*;

mod editor;
pub use editor::*;

mod document_docking;
pub use document_docking::*;

mod tool_docking;
pub use tool_docking::*;

mod top_bar;
pub use top_bar::*;
mod messages;
pub use messages::*;

mod tools;
pub use tools::*;

pub type TerminalResult<T> = Result<T, Box<dyn Error>>;

pub trait ModalDialog {
    fn show(&mut self, ctx: &egui::Context) -> bool;

    fn should_commit(&self) -> bool;

    fn commit(&self, _editor: &mut AnsiEditor) -> TerminalResult<bool> {
        Ok(true)
    }

    fn commit_self(&self, _window: &mut MainWindow) -> TerminalResult<bool> {
        Ok(true)
    }
}
