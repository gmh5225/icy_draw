use std::{
    cell::RefCell,
    path::PathBuf,
    rc::Rc,
    sync::{Arc, Mutex},
};

use eframe::egui;
use icy_engine::{BitFont, Selection, TheDrawFont};

use crate::{
    MainWindow, NewFileDialog, OpenFileDialog, SaveFileDialog, SelectCharacterDialog,
    SelectOutlineDialog,
};

pub enum Message {
    NewFile,
    OpenFile,
    SaveFile,
    SaveFileAs,
    ExportFile,
    ShowOutlineDialog,

    NewLayer,
    EditLayer(usize),
    DeleteLayer(usize),
    MoveLayerUp(usize),
    MoveLayerDown(usize),
    ToggleVisibility(usize),
    SelectLayer(usize),
    Undo,
    Redo,
    EditSauce,
    SetCanvasSize,
    SelectAll,
    Deselect,
    DeleteSelection,

    ShowAboutDialog,
    ShowCharacterSelectionDialog(Rc<RefCell<char>>),
    SelectFontDialog(Arc<Mutex<Vec<TheDrawFont>>>, Arc<Mutex<i32>>),
    ShowError(String),
    SetFontPage(usize),
}

pub const CTRL_SHIFT: egui::Modifiers = egui::Modifiers {
    alt: false,
    ctrl: true,
    shift: true,
    mac_cmd: false,
    command: false,
};

impl MainWindow {
    pub fn handle_message(&mut self, msg_opt: Option<Message>) {
        if msg_opt.is_none() {
            return;
        }
        match msg_opt.unwrap() {
            Message::NewFile => {
                self.open_dialog(NewFileDialog::default());
            }

            Message::OpenFile => {
                self.open_dialog(OpenFileDialog::default());
            }

            Message::SaveFile => {
                if let Some(doc) = self.get_active_pane() {
                    let mut save_as = true;
                    if let Some(str) = &doc.full_path {
                        let path = PathBuf::from(str);
                        if let Some(ext) = path.extension() {
                            if ext == "icd" {
                                doc.doc.lock().unwrap().save(str).unwrap();
                                save_as = false;
                            }
                        }
                    }
                    if save_as {
                        self.handle_message(Some(Message::SaveFileAs))
                    }
                }
            }
            Message::SaveFileAs => {
                if self.get_active_document().is_some() {
                    self.open_dialog(SaveFileDialog::default());
                }
            }
            Message::ExportFile => {
                let mut buffer_opt = self.get_ansi_editor();
                let view = buffer_opt.unwrap().buffer_view.clone();
                self.open_dialog(crate::ExportFileDialog::new(&view.lock().buf));
            }
            Message::ShowOutlineDialog => {
                self.open_dialog(SelectOutlineDialog::default());
            }
            Message::Undo => {
                if let Some(editor) = self.get_ansi_editor() {
                    editor.undo();
                    editor.buffer_view.lock().redraw_view();
                }
            }
            Message::Redo => {
                if let Some(editor) = self.get_ansi_editor() {
                editor.redo();
                    editor.buffer_view.lock().redraw_view();
                }
            
            }

            Message::SelectAll => {
                if let Some(editor) = self.get_ansi_editor() {
                    let buf = &mut editor.buffer_view.lock();
                        let w = buf.buf.get_width();
                        let h = buf.buf.get_line_count();

                        buf.set_selection(Selection::from_rectangle(0.0, 0.0, w as f32, h as f32));
                    }
            }
            Message::Deselect => {
                if let Some(editor) = self.get_ansi_editor() {
                    editor.buffer_view.lock().clear_selection();
                        editor.redraw_view();
                    }
            }

            Message::DeleteSelection => {
                if let Some(editor) = self.get_ansi_editor() {
                    if editor.buffer_view.lock().get_selection().is_some() {
                            editor.delete_selection();
                            editor.redraw_view();
                        }
                }
            }

            Message::ShowCharacterSelectionDialog(ch) => {
                if let Some(editor) = self.get_ansi_editor() {
                    let buf = editor.buffer_view.clone();
                        self.open_dialog(SelectCharacterDialog::new(buf, ch));
                    }
            }
            Message::SelectFontDialog(fonts, selected_font) => {
                self.open_dialog(crate::SelectFontDialog::new(fonts, selected_font));
            }

            Message::EditSauce => {
                let mut buffer_opt = self.get_ansi_editor() ;
       
                let view = buffer_opt.unwrap().buffer_view.clone();
                self.open_dialog(crate::EditSauceDialog::new(&view.lock().buf));
            }
            Message::SetCanvasSize => {
                let mut buffer_opt = self.get_ansi_editor();
                let view = buffer_opt.unwrap().buffer_view.clone();
                self.open_dialog(crate::SetCanvasSizeDialog::new(&view.lock().buf));
            }

            Message::EditLayer(i) => {
                let editor = self.get_ansi_editor()
                    .unwrap();
                let buffer_view = editor.buffer_view.clone();
                self.open_dialog(crate::EditLayerDialog::new(&buffer_view.lock().buf, i));
            }
            Message::NewLayer => {
                let editor = self.get_ansi_editor()
                    .unwrap();
                let buf = &mut editor.buffer_view.lock().buf;
                let size = buf.get_buffer_size();
                let mut new_layer = icy_engine::Layer::new("New Layer", size);
                new_layer.has_alpha_channel = true;
                if buf.layers.is_empty() {
                    new_layer.has_alpha_channel = false;
                }

                buf.layers.insert(0, new_layer);
            }
            Message::MoveLayerUp(cur_layer) => {
                let editor = self.get_ansi_editor()
                    .unwrap();

                editor
                    .buffer_view
                    .lock()
                    .buf
                    .layers
                    .swap(cur_layer, cur_layer - 1);
                editor.cur_layer -= 1;
            }
            Message::MoveLayerDown(cur_layer) => {
                let editor = self.get_ansi_editor()
                    .unwrap();

                editor
                    .buffer_view
                    .lock()
                    .buf
                    .layers
                    .swap(cur_layer, cur_layer + 1);
                editor.cur_layer += 1;
            }
            Message::DeleteLayer(cur_layer) => {
                let editor = self.get_ansi_editor()
                    .unwrap();
                editor.buffer_view.lock().buf.layers.remove(cur_layer);
                editor.cur_layer = editor.cur_layer.clamp(
                    0,
                    editor.buffer_view.lock().buf.layers.len().saturating_sub(1),
                );
            }
            Message::ToggleVisibility(cur_layer) => {
                let editor = self.get_ansi_editor()
                    .unwrap();
                let is_visible = editor.buffer_view.lock().buf.layers[cur_layer].is_visible;
                editor.buffer_view.lock().buf.layers[cur_layer].is_visible = !is_visible;
            }
            Message::SelectLayer(cur_layer) => {
                let editor = self .get_ansi_editor()
                    .unwrap();
                editor.cur_layer = cur_layer;
            }

            Message::SetFontPage(page) => {
                let editor = self.get_ansi_editor()
                    .unwrap();
                editor.buffer_view.lock().caret.set_font_page(page);

                let buf = &mut editor.buffer_view.lock().buf;
                if buf.get_font(page).is_none() {
                    if let Some(font_name) =
                        icy_engine::parsers::ansi::constants::ANSI_FONT_NAMES.get(page)
                    {
                        match BitFont::from_name(font_name) {
                            Ok(font) => {
                                buf.set_font(page, font);
                            }
                            Err(err) => {
                                log::error!("Failed to load font: {err}");
                            }
                        }
                    }
                }
            }

            Message::ShowAboutDialog => {
                self.open_dialog(crate::AboutDialog::default());
            }

            Message::ShowError(msg) => {
                log::error!("{msg}");
                self.toasts.error(msg);
            }
        }
    }
}
