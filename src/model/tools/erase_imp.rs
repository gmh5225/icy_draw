use eframe::egui;
use i18n_embed_fl::fl;
use icy_engine::{editor::AtomicUndoGuard, AttributedChar, TextAttribute};
use icy_engine_egui::TerminalCalc;

use crate::{AnsiEditor, Event, Message};

use super::{Position, Tool};

#[derive(PartialEq, Eq)]
pub enum EraseType {
    Shade,
    Solid,
}

pub struct EraseTool {
    pub size: i32,
    pub brush_type: EraseType,
    pub undo_op: Option<AtomicUndoGuard>,
}

impl EraseTool {
    fn eraser(&self, editor: &mut AnsiEditor, pos: Position) {
        let mid = Position::new(-(self.size / 2), -(self.size / 2));

        let center = pos + mid;
        let gradient = ['\u{00DB}', '\u{00B2}', '\u{00B1}', '\u{00B0}', ' '];

        for y in 0..self.size {
            for x in 0..self.size {
                match self.brush_type {
                    EraseType::Shade => {
                        let ch = editor.get_char_from_cur_layer(center + Position::new(x, y));

                        let mut attribute = ch.attribute;

                        let mut char_code = gradient[0];
                        let mut found = false;
                        if ch.ch == gradient[gradient.len() - 1] {
                            char_code = gradient[gradient.len() - 1];
                            attribute = TextAttribute::default();
                            found = true;
                        } else {
                            for i in 0..gradient.len() - 1 {
                                if ch.ch == gradient[i] {
                                    char_code = gradient[i + 1];
                                    found = true;
                                    break;
                                }
                            }
                        }

                        if found {
                            editor.set_char(
                                center + Position::new(x, y),
                                AttributedChar::new(char_code, attribute),
                            );
                        }
                    }
                    EraseType::Solid => {
                        editor.set_char(
                            center + Position::new(x, y),
                            AttributedChar::new(' ', TextAttribute::default()),
                        );
                    }
                }
            }
        }
    }
}

impl Tool for EraseTool {
    fn get_icon_name(&self) -> &'static egui_extras::RetainedImage {
        &super::icons::ERASER_SVG
    }

    fn use_caret(&self) -> bool {
        false
    }

    fn show_ui(
        &mut self,
        _ctx: &egui::Context,
        ui: &mut egui::Ui,
        _buffer_opt: &AnsiEditor,
    ) -> Option<Message> {
        ui.horizontal(|ui| {
            ui.label(fl!(crate::LANGUAGE_LOADER, "tool-size-label"));
            ui.add(
                egui::DragValue::new(&mut self.size)
                    .clamp_range(1..=20)
                    .speed(1),
            );
        });
        ui.radio_value(
            &mut self.brush_type,
            EraseType::Solid,
            fl!(crate::LANGUAGE_LOADER, "tool-solid"),
        );
        ui.radio_value(
            &mut self.brush_type,
            EraseType::Shade,
            fl!(crate::LANGUAGE_LOADER, "tool-shade"),
        );
        None
    }

    fn handle_click(
        &mut self,
        editor: &mut AnsiEditor,
        button: i32,
        pos: Position,
    ) -> super::Event {
        if button == 1 {
            let _undo = editor.begin_atomic_undo(fl!(crate::LANGUAGE_LOADER, "undo-eraser"));

            self.eraser(editor, pos);
        }
        super::Event::None
    }

    fn handle_drag(
        &mut self,
        _ui: &egui::Ui,
        response: egui::Response,
        editor: &mut AnsiEditor,
        _calc: &TerminalCalc,
    ) -> egui::Response {
        self.eraser(editor, editor.drag_pos.cur);
        response
    }

    fn handle_drag_begin(&mut self, editor: &mut AnsiEditor) -> Event {
        self.undo_op = Some(editor.begin_atomic_undo(fl!(crate::LANGUAGE_LOADER, "undo-eraser")));
        Event::None
    }

    fn handle_drag_end(
        &mut self,
        _editor: &mut AnsiEditor,
    ) -> Event {
        self.undo_op = None;
        Event::None
    }
}
