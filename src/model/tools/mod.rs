pub mod brush_imp;
pub mod click_imp;
pub mod draw_ellipse_filled_imp;
pub mod draw_ellipse_imp;
pub mod draw_rectangle_filled_imp;
pub mod draw_rectangle_imp;
pub mod erase_imp;
pub mod fill_imp;
pub mod flip_imp;
pub mod font_imp;
pub mod line_imp;
pub mod move_layer_imp;
pub mod paste_tool;
pub mod pencil_imp;
pub mod pipette_imp;

mod icons;

use eframe::egui::{self, Response};
use egui_extras::RetainedImage;
use icy_engine::{AttributedChar, Position, TextAttribute};
use icy_engine_egui::TerminalCalc;
pub use scan_lines::*;

use crate::{AnsiEditor, Event, Message};
pub mod scan_lines;

#[derive(Copy, Clone, Debug)]
pub enum MKey {
    Character(u16),
    Down,
    Up,
    Left,
    Right,
    PageDown,
    PageUp,
    Home,
    End,
    Return,
    Delete,
    Insert,
    Backspace,
    Tab,
    Escape,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
}

#[derive(Copy, Clone, Debug)]
pub enum MModifiers {
    None,
    Shift,
    Alt,
    Control,
}

impl MModifiers {
    pub fn is_shift(self) -> bool {
        matches!(self, MModifiers::Shift)
    }

    pub fn is_alt(self) -> bool {
        matches!(self, MModifiers::Alt)
    }

    pub fn is_control(self) -> bool {
        matches!(self, MModifiers::Control)
    }
}

#[derive(Default, Clone, Copy)]
pub struct DragPos {
    pub start_abs: Position,
    pub cur_abs: Position,
    pub start: Position,
    pub cur: Position
} 

pub trait Tool {
    fn get_icon_name(&self) -> &'static RetainedImage;

    fn use_caret(&self) -> bool {
        true
    }

    fn is_visible(&self) -> bool {
        true
    }

    fn is_exclusive(&self) -> bool {
        false
    }

    fn use_selection(&self) -> bool {
        true
    }

    fn show_ui(
        &mut self,
        ctx: &egui::Context,
        ui: &mut egui::Ui,
        buffer_opt: &AnsiEditor,
    ) -> Option<Message>;

    fn handle_key(&mut self, editor: &mut AnsiEditor, key: MKey, modifier: MModifiers) -> Event {
        // TODO Keys:

        // Tab - Next tab
        // Shift+Tab - Prev tab

        // ctrl+pgup  - upper left corner
        // ctrl+pgdn  - lower left corner
        let pos = editor.buffer_view.lock().get_caret().get_position();
        match key {
            MKey::Down => {
                if let MModifiers::Control = modifier {
                    let fg = (editor
                        .buffer_view
                        .lock()
                        .get_caret()
                        .get_attribute()
                        .get_foreground()
                        + 14)
                        % 16;
                    editor
                        .buffer_view
                        .lock()
                        .get_caret()
                        .get_attribute()
                        .set_foreground(fg);
                } else {
                    editor.set_caret(pos.x, pos.y + 1);
                }
            }
            MKey::Up => {
                if let MModifiers::Control = modifier {
                    let fg = (editor
                        .buffer_view
                        .lock()
                        .get_caret()
                        .get_attribute()
                        .get_foreground()
                        + 1)
                        % 16;
                    editor
                        .buffer_view
                        .lock()
                        .get_caret()
                        .get_attribute()
                        .set_foreground(fg);
                } else {
                    editor.set_caret(pos.x, pos.y - 1);
                }
            }
            MKey::Left => {
                // TODO: ICE Colors
                if let MModifiers::Control = modifier {
                    let bg = (editor
                        .buffer_view
                        .lock()
                        .get_caret()
                        .get_attribute()
                        .get_background()
                        + 7)
                        % 8;
                    editor
                        .buffer_view
                        .lock()
                        .get_caret()
                        .get_attribute()
                        .set_background(bg);
                } else {
                    editor.set_caret(pos.x - 1, pos.y);
                }
            }
            MKey::Right => {
                // TODO: ICE Colors
                if let MModifiers::Control = modifier {
                    let bg = (editor
                        .buffer_view
                        .lock()
                        .get_caret()
                        .get_attribute()
                        .get_background()
                        + 1)
                        % 8;
                    editor
                        .buffer_view
                        .lock()
                        .get_caret()
                        .get_attribute()
                        .set_background(bg);
                } else {
                    editor.set_caret(pos.x + 1, pos.y);
                }
            }
            MKey::PageDown => {
                // TODO
                println!("pgdn");
            }
            MKey::PageUp => {
                // TODO
                println!("pgup");
            }
            /*
            MKey::Tab => {
                let tab_size = unsafe { crate::WORKSPACE.settings.tab_size } ;
                if let MModifiers::Control = modifier {
                    let tabs = max(0, (pos.x / tab_size) - 1);
                    let next_tab = tabs * tab_size;
                    editor.set_caret(next_tab, pos.y);
                } else {
                    let tabs = 1 + pos.x / tab_size;
                    let next_tab = min(editor.get_buffer().width as i32 - 1, tabs * tab_size);
                    editor.set_caret(next_tab, pos.y);
                }
            }
            MKey::Home  => {
                if let MModifiers::Control = modifier {
                    for i in 0..editor.get_buffer().width {
                        if !editor.get_char_from_cur_layer(pos.with_x(i as i32)).unwrap_or_default().is_transparent() {
                            editor.set_caret(i as i32, pos.y);
                            return Event::None;
                        }
                    }
                }
                editor.set_caret(0, pos.y);
            }
            MKey::End => {
                if let MModifiers::Control = modifier {
                    for i in (0..editor.get_buffer().width).rev()  {
                        if !editor.get_char_from_cur_layer(pos.with_x(i as i32)).unwrap_or_default().is_transparent() {
                            editor.set_caret(i as i32, pos.y);
                            return Event::None;
                        }
                    }
                }
                let w = editor.get_buffer().width as i32;
                editor.set_caret(w - 1, pos.y);
            }*/
            MKey::Return => {
                editor.set_caret(0, pos.y + 1);
            }
            MKey::Delete => {
                if editor.buffer_view.lock().get_selection().is_some() {
                    editor.delete_selection();
                } else {
                    let pos = editor.get_caret_position();
                    let end = editor.buffer_view.lock().get_width() - 1;
                    for i in pos.x..end {
                        let next = editor.get_char_from_cur_layer(Position::new(i + 1, pos.y));
                        editor.set_char(Position::new(i, pos.y), next);
                    }
                    let last_pos = Position::new(editor.buffer_view.lock().get_width() - 1, pos.y);
                    editor.set_char(last_pos, AttributedChar::invisible());
                }
            }
            MKey::Insert => {
                editor.buffer_view.lock().get_caret_mut().insert_mode =
                    !editor.buffer_view.lock().get_caret().insert_mode;
            }
            MKey::Backspace => {
                editor.buffer_view.lock().clear_selection();
                let pos = editor.get_caret_position();
                if pos.x > 0 {
                    /* if (caret.fontMode() && FontTyped && cpos > 0)  {
                        caret.getX() -= CursorPos[cpos] - 1;
                        for (a=0;a<=CursorPos[cpos];a++)
                        for (b=0;b<=FontLibrary::getInstance().maxY;b++) {
                            getCurrentBuffer()->getCharacter(caret.getLogicalY() + b, caret.getLogicalX()+a) = getUndoBuffer()->getCharacter(caret.getLogicalY() + b, caret.getLogicalX()+a);
                            getCurrentBuffer()->getAttribute(caret.getLogicalY() + b, caret.getLogicalX()+a) = getUndoBuffer()->getAttribute(caret.getLogicalY() + b, caret.getLogicalX()+a);
                        }
                        cpos--;
                    } else {*/
                    editor.set_caret_position(pos + Position::new(-1, 0));
                    if editor.buffer_view.lock().get_caret().insert_mode {
                        let end = editor.buffer_view.lock().get_width() - 1;
                        for i in pos.x..end {
                            let next = editor.get_char_from_cur_layer(Position::new(i + 1, pos.y));
                            editor.set_char(Position::new(i, pos.y), next);
                        }
                        let last_pos =
                            Position::new(editor.buffer_view.lock().get_width() - 1, pos.y);
                        editor.set_char(last_pos, AttributedChar::invisible());
                    } else {
                        let pos = editor.get_caret_position();
                        editor.set_char(pos, AttributedChar::invisible());
                    }
                }
            }

            MKey::Character(ch) => {
                editor.buffer_view.lock().clear_selection();
                /*        if let MModifiers::Alt = modifier {
                    match key_code {
                        MKeyCode::KeyI => editor.insert_line(pos.y),
                        MKeyCode::KeyU => editor.pickup_color(pos),
                        MKeyCode::KeyY => editor.delete_line(pos.y),
                        MKeyCode::Unknown => {}
                    }
                    return Event::None;
                }*/

                editor.type_key(unsafe { char::from_u32_unchecked(ch as u32) });
            }

            MKey::F1 => {
                handle_outline_insertion(editor, modifier, 0);
            }
            MKey::F2 => {
                handle_outline_insertion(editor, modifier, 1);
            }
            MKey::F3 => {
                handle_outline_insertion(editor, modifier, 2);
            }
            MKey::F4 => {
                handle_outline_insertion(editor, modifier, 3);
            }
            MKey::F5 => {
                handle_outline_insertion(editor, modifier, 4);
            }
            MKey::F6 => {
                handle_outline_insertion(editor, modifier, 5);
            }
            MKey::F7 => {
                handle_outline_insertion(editor, modifier, 6);
            }
            MKey::F8 => {
                handle_outline_insertion(editor, modifier, 7);
            }
            MKey::F9 => {
                handle_outline_insertion(editor, modifier, 8);
            }
            MKey::F10 => {
                handle_outline_insertion(editor, modifier, 9);
            }
            MKey::Escape => {
                editor.buffer_view.lock().clear_selection();
            }
            _ => {}
        }
        Event::None
    }

    fn handle_click(
        &mut self,
        _buffer_view: &mut AnsiEditor,
        _button: i32,
        _pos: Position,
    ) -> Event {
        Event::None
    }

    fn handle_drag_begin(&mut self, _editor: &mut AnsiEditor) -> Event {
        Event::None
    }

    fn handle_drag(
        &mut self,
        _ui: &egui::Ui,
        response: Response,
        _editor: &mut AnsiEditor,
        _calc: &TerminalCalc
    ) -> Response {
        response
    }

    fn handle_hover(
        &mut self,
        _ui: &egui::Ui,
        response: Response,
        _editor: &mut AnsiEditor,
        _cur: Position,
    ) -> Response {
        response
    }

    fn handle_drag_end(
        &mut self,
        _editor: &mut AnsiEditor,
    ) -> Event {
        Event::None
    }
}

fn handle_outline_insertion(editor: &mut AnsiEditor, modifier: MModifiers, outline: usize) {
    if let MModifiers::Control = modifier {
        editor.set_cur_outline(outline);
        return;
    }

    if outline < 5 {
        if let MModifiers::Shift = modifier {
            editor.set_cur_outline(10 + outline);
            return;
        }
    }
    editor.buffer_view.lock().clear_selection();
    let ch = editor.get_outline_char_code(outline);
    if let Ok(ch) = ch {
        editor.type_key(unsafe { char::from_u32_unchecked(ch as u32) });
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DrawMode {
    Line,
    Char,
    Shade,
    Colorize,
}

trait Plottable {
    fn get_draw_mode(&self) -> DrawMode;

    fn get_use_fore(&self) -> bool;
    fn get_use_back(&self) -> bool;
    fn get_char_code(&self) -> char;
}

fn plot_point(editor: &AnsiEditor, tool: &dyn Plottable, pos: Position) {
    let ch = editor.get_char_from_cur_layer(pos);
    let editor_attr = editor.buffer_view.lock().get_caret().get_attribute();
    let mut attribute = ch.attribute;
    if !ch.is_visible() {
        attribute = TextAttribute::default();
    }
    if tool.get_use_back() {
        attribute.set_background(editor_attr.get_background());
    }
    if tool.get_use_fore() {
        attribute.set_foreground(editor_attr.get_foreground());
    }

    attribute.set_font_page(editor_attr.get_font_page());

    match tool.get_draw_mode() {
        DrawMode::Line => {
            if let Some(layer) = editor
                .buffer_view
                .lock()
                .get_buffer_mut()
                .get_overlay_layer()
            {
                layer.set_char(
                    pos,
                    AttributedChar::new(unsafe { char::from_u32_unchecked(219) }, attribute),
                );
            }
        }
        DrawMode::Char => {
            if let Some(layer) = editor
                .buffer_view
                .lock()
                .get_buffer_mut()
                .get_overlay_layer()
            {
                layer.set_char(pos, AttributedChar::new(tool.get_char_code(), attribute));
            }
        }
        DrawMode::Shade => {
            let mut char_code = SHADE_GRADIENT[0];
            if ch.ch == SHADE_GRADIENT[SHADE_GRADIENT.len() - 1] {
                char_code = SHADE_GRADIENT[SHADE_GRADIENT.len() - 1];
            } else {
                for i in 0..SHADE_GRADIENT.len() - 1 {
                    if ch.ch == SHADE_GRADIENT[i] {
                        char_code = SHADE_GRADIENT[i + 1];
                        break;
                    }
                }
            }
            if let Some(layer) = editor
                .buffer_view
                .lock()
                .get_buffer_mut()
                .get_overlay_layer()
            {
                layer.set_char(pos, AttributedChar::new(char_code, attribute));
            }
        }
        DrawMode::Colorize => {
            if let Some(layer) = editor
                .buffer_view
                .lock()
                .get_buffer_mut()
                .get_overlay_layer()
            {
                layer.set_char(pos, AttributedChar::new(ch.ch, attribute));
            }
        }
    }
}

pub static SHADE_GRADIENT: [char; 4] = ['\u{00B0}', '\u{00B1}', '\u{00B2}', '\u{00DB}'];
