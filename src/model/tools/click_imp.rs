use eframe::egui;
use egui_extras::RetainedImage;
use icy_engine::Selection;

use crate::{AnsiEditor, Message};

use super::{Event, Position, Tool};

pub struct ClickTool {}

impl Tool for ClickTool {
    fn get_icon_name(&self) -> &'static RetainedImage {
        &super::icons::CURSOR_SVG
    }

    fn show_ui(
        &mut self,
        _ctx: &egui::Context,
        _ui: &mut egui::Ui,
        _buffer_opt: &AnsiEditor,
    ) -> Option<Message> {
        None
    }

    fn handle_click(&mut self, editor: &mut AnsiEditor, button: i32, pos: Position) -> Event {
        if button == 1 {
            editor.set_caret_position(pos);
            editor.buffer_view.lock().clear_selection();
        }
        Event::None
    }

    fn handle_drag(
        &mut self,
        _ui: &egui::Ui,
        response: egui::Response,
        editor: &mut AnsiEditor,
        start: Position,
        cur: Position,
    ) -> egui::Response {
        if start == cur {
            editor.buffer_view.lock().clear_selection();
        } else {
            editor
                .buffer_view
                .lock()
                .set_selection(Selection::from_rectangle(
                    start.x.min(cur.x) as f32,
                    start.y.min(cur.y) as f32,
                    (cur.x - start.x).abs() as f32,
                    (cur.y - start.y).abs() as f32,
                ));
        }
        response
    }

    fn handle_hover(
        &mut self,
        ui: &egui::Ui,
        response: egui::Response,
        _editor: &mut AnsiEditor,
        _cur: Position,
    ) -> egui::Response {
        ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::Text);
        response
    }

    fn handle_drag_end(
        &mut self,
        editor: &mut AnsiEditor,
        start: Position,
        cur: Position,
    ) -> Event {
        let mut cur = cur;
        if start < cur {
            cur += Position::new(1, 1);
        }

        if start == cur {
            editor.buffer_view.lock().clear_selection();
        }

        Event::None
    }
}
