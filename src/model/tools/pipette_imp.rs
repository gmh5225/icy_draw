use eframe::egui;

use crate::{AnsiEditor, Message};

use super::{Event, Position, Tool};
pub struct PipetteTool {}

impl Tool for PipetteTool {
    fn get_icon_name(&self) -> &'static egui_extras::RetainedImage {
        &super::icons::DROPPER_SVG
    }

    fn use_caret(&self) -> bool {
        false
    }

    fn use_selection(&self) -> bool {
        false
    }

    fn show_ui(
        &mut self,
        _ctx: &egui::Context,
        _ui: &mut egui::Ui,
        _editor_opt: Option<&AnsiEditor>,
    ) -> Option<Message> {
        None
    }

    fn handle_hover(
        &mut self,
        _ui: &egui::Ui,
        response: egui::Response,
        _editor: &mut AnsiEditor,
        _cur: Position,
        _cur_abs: Position,
    ) -> egui::Response {
        response.on_hover_cursor(egui::CursorIcon::Crosshair)
    }

    fn handle_click(
        &mut self,
        editor: &mut AnsiEditor,
        button: i32,
        pos: Position,
        _pos_abs: Position,
        _response: &egui::Response,
    ) -> Event {
        if button == 1 {
            let ch = editor.get_char(pos);
            editor.set_caret_attribute(ch.attribute);
        }
        Event::None
    }
}
