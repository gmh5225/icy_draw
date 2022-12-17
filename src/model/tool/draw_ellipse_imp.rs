use std::sync::{Arc, Mutex};

use eframe::egui;
use i18n_embed_fl::fl;
use icy_engine::{TextAttribute, Rectangle};

use crate::ansi_editor::BufferView;

use super::{ DrawMode, Event, Plottable, Position, Tool, ScanLines, brush_imp::draw_glyph, line_imp::set_half_block};

pub struct DrawEllipseTool {
    pub draw_mode: DrawMode,

    pub use_fore: bool,
    pub use_back: bool,
    pub attr: TextAttribute,
    pub char_code: char,
    pub font_page: usize,
}

impl Plottable for DrawEllipseTool {
    fn get_draw_mode(&self) -> DrawMode {
        self.draw_mode
    }
    fn get_use_fore(&self) -> bool {
        self.use_fore
    }
    fn get_use_back(&self) -> bool {
        self.use_back
    }
    fn get_char_code(&self) -> char {
        self.char_code
    }
}

impl Tool for DrawEllipseTool {
    fn get_icon_name(&self) -> &'static egui_extras::RetainedImage { &super::icons::ELLIPSE_OUTLINE_SVG }

    fn use_caret(&self) -> bool {
        false
    }
    fn use_selection(&self) -> bool {
        false
    }

    fn show_ui(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui, buffer_opt: Option<std::sync::Arc<std::sync::Mutex<crate::ui::ansi_editor::BufferView>>>)
    {
        ui.vertical_centered(|ui| {
            ui.horizontal(|ui| {
                if ui.selectable_label(self.use_fore, fl!(crate::LANGUAGE_LOADER, "tool-fg")).clicked() {
                    self.use_fore = !self.use_fore;
                }
                if ui.selectable_label(self.use_back, fl!(crate::LANGUAGE_LOADER, "tool-bg")).clicked() {
                    self.use_back = !self.use_back;
                }
            });
        });

        ui.radio_value(&mut self.draw_mode, DrawMode::Line, fl!(crate::LANGUAGE_LOADER, "tool-line"));
        ui.horizontal(|ui| {
            ui.radio_value(&mut self.draw_mode, DrawMode::Char, fl!(crate::LANGUAGE_LOADER, "tool-character"));

            if let Some(b) = &buffer_opt {
                ui.add(draw_glyph(b.clone(), self.char_code, self.font_page));
            }
        });
        ui.radio_value(&mut self.draw_mode, DrawMode::Shade, fl!(crate::LANGUAGE_LOADER, "tool-shade"));
        ui.radio_value(&mut self.draw_mode, DrawMode::Colorize, fl!(crate::LANGUAGE_LOADER, "tool-colorize"));
    }

    fn handle_drag(&self, buffer_view: Arc<Mutex<BufferView>>, mut start: Position, mut cur: Position) -> Event {
        if let Some(layer) = buffer_view.lock().unwrap().editor.get_overlay_layer() {
            layer.clear();
        }

        let mut lines = ScanLines::new(1);

        if self.draw_mode == DrawMode::Line {
            start.y *= 2;
            cur.y *= 2;
        }

        if start < cur {
            lines.add_ellipse(Rectangle::from_pt( start, cur));
        } else {
            lines.add_ellipse(Rectangle::from_pt(cur, start));
        }

        let col = buffer_view.lock().unwrap().editor.caret.get_attribute().get_foreground();
        let buffer_view = buffer_view.clone();
        let draw = move |rect: Rectangle| {
            let editor = &mut buffer_view.lock().unwrap().editor;
            for y in 0..rect.size.height {
                for x in 0..rect.size.width {
                    set_half_block(editor, Position::new(rect.start.x + x, rect.start.y + y ), col);
                }
            }
        };
        lines.outline(draw);
        Event::None
    }

    fn handle_drag_end(
        &self,
        buffer_view: Arc<Mutex<BufferView>>,
        start: Position,
        cur: Position,
    ) -> Event {
        let editor = &mut buffer_view.lock().unwrap().editor;
        if start == cur {
            editor.buf.remove_overlay();
        } else {
            editor.join_overlay();
        }
        Event::None
    }
}
