use eframe::egui;
use i18n_embed_fl::fl;
use icy_engine_egui::TerminalCalc;

use crate::{
    paint::{draw_ellipse, BrushMode, ColorMode},
    AnsiEditor, Event, Message,
};

use super::{Position, Tool};

pub struct DrawEllipseTool {
    draw_mode: BrushMode,
    color_mode: ColorMode,
    char_code: std::rc::Rc<std::cell::RefCell<char>>,
    old_pos: Position,
}

impl Default for DrawEllipseTool {
    fn default() -> Self {
        Self {
            draw_mode: BrushMode::HalfBlock,
            color_mode: crate::paint::ColorMode::Both,
            char_code: std::rc::Rc::new(std::cell::RefCell::new('\u{00B0}')),
            old_pos: Position::default(),
        }
    }
}

impl Tool for DrawEllipseTool {
    fn get_icon(&self) -> &egui::Image<'static> {
        &super::icons::ELLIPSE_OUTLINE_SVG
    }

    fn tool_name(&self) -> String {
        fl!(crate::LANGUAGE_LOADER, "tool-ellipse_name")
    }

    fn tooltip(&self) -> String {
        fl!(crate::LANGUAGE_LOADER, "tool-ellipse_tooltip")
    }

    fn use_caret(&self) -> bool {
        false
    }
    fn use_selection(&self) -> bool {
        false
    }

    fn show_ui(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui, editor_opt: Option<&mut AnsiEditor>) -> Option<Message> {
        self.color_mode.show_ui(ui);
        self.draw_mode
            .show_ui(ui, editor_opt, self.char_code.clone(), crate::paint::BrushUi::HideOutline)
    }

    fn handle_hover(&mut self, _ui: &egui::Ui, response: egui::Response, _editor: &mut AnsiEditor, _cur: Position, _cur_abs: Position) -> egui::Response {
        response.on_hover_cursor(egui::CursorIcon::Crosshair)
    }

    fn handle_drag_begin(&mut self, _editor: &mut AnsiEditor, _response: &egui::Response) -> Event {
        self.old_pos = Position::new(-1, -1);
        Event::None
    }

    fn handle_drag(&mut self, _ui: &egui::Ui, response: egui::Response, editor: &mut AnsiEditor, _calc: &TerminalCalc) -> egui::Response {
        let p2 = editor.half_block_click_pos;
        if self.old_pos == p2 {
            return response;
        }
        self.old_pos = p2;

        editor.clear_overlay_layer();
        let p1 = editor.drag_pos.start_half_block;
        let start = Position::new(p1.x.min(p2.x), p1.y.min(p2.y));
        let end = Position::new(p1.x.max(p2.x), p1.y.max(p2.y));
        draw_ellipse(&mut editor.buffer_view.lock(), start, end, self.draw_mode.clone(), self.color_mode);
        response
    }

    fn handle_drag_end(&mut self, editor: &mut AnsiEditor) -> Option<Message> {
        if editor.drag_pos.start == editor.drag_pos.cur {
            editor.buffer_view.lock().get_buffer_mut().remove_overlay();
        } else {
            editor.join_overlay(fl!(crate::LANGUAGE_LOADER, "undo-draw-ellipse"));
        }
        None
    }
}
