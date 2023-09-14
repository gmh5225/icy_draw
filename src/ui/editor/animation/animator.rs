use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use eframe::epaint::mutex::Mutex;
use icy_engine::{AttributedChar, Buffer, Caret, TextPane};
use mlua::{Lua, UserData, Value};

#[derive(Default)]
pub struct Animator {
    pub scene: Option<Buffer>,
    pub frames: Vec<Buffer>,

    pub buffers: Vec<Buffer>,
}

struct LuaBuffer {
    caret: Caret,
    buffer: Buffer,
}

impl UserData for LuaBuffer {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("height", |_, this| Ok(this.buffer.get_height()));
        fields.add_field_method_set("height", |_, this, val| {
            this.buffer.set_height(val);
            Ok(())
        });
        fields.add_field_method_get("width", |_, this| Ok(this.buffer.get_width()));
        fields.add_field_method_set("width", |_, this, val| {
            this.buffer.set_width(val);
            Ok(())
        });

        fields.add_field_method_get("font_page", |_, this| Ok(this.caret.get_font_page()));
        fields.add_field_method_set("font_page", |_, this, val| {
            this.caret.set_font_page(val);
            Ok(())
        });

        fields.add_field_method_get("fg", |_, this| {
            Ok(this.caret.get_attribute().get_foreground())
        });
        fields.add_field_method_set("fg", |_, this, val| {
            let mut attr = this.caret.get_attribute();
            attr.set_foreground(val);
            this.caret.set_attr(attr);
            Ok(())
        });

        fields.add_field_method_get("bg", |_, this| {
            Ok(this.caret.get_attribute().get_background())
        });
        fields.add_field_method_set("bg", |_, this, val| {
            let mut attr = this.caret.get_attribute();
            attr.set_background(val);
            this.caret.set_attr(attr);
            Ok(())
        });

        fields.add_field_method_get("layer_count", |_, this| Ok(this.buffer.layers.len()));
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("set_fg", |_, this, (r, g, b): (u8, u8, u8)| {
            let color = this.buffer.palette.insert_color_rgb(r, g, b);
            this.caret.set_foreground(color);
            Ok(())
        });

        methods.add_method_mut("set_bg", |_, this, (r, g, b): (u8, u8, u8)| {
            let color = this.buffer.palette.insert_color_rgb(r, g, b);
            this.caret.set_background(color);
            Ok(())
        });

        methods.add_method_mut(
            "set_char",
            |_, this, (layer, x, y, ch): (usize, i32, i32, u32)| {
                if layer < this.buffer.layers.len() {
                    this.buffer.layers[layer].set_char(
                        (x, y),
                        AttributedChar::new(
                            unsafe { std::char::from_u32_unchecked(ch) },
                            this.caret.get_attribute(),
                        ),
                    );
                    Ok(())
                } else {
                    Err(mlua::Error::SyntaxError {
                        message: format!(
                            "Layer {} out of range (0..<{})",
                            layer,
                            this.buffer.layers.len()
                        ),
                        incomplete_input: false,
                    })
                }
            },
        );

        methods.add_method_mut("get_char", |_, this, (layer, x, y): (usize, i32, i32)| {
            if layer < this.buffer.layers.len() {
                let ch = this.buffer.layers[layer].get_char((x, y));
                Ok(ch.ch as u32)
            } else {
                Err(mlua::Error::SyntaxError {
                    message: format!(
                        "Layer {} out of range (0..<{})",
                        layer,
                        this.buffer.layers.len()
                    ),
                    incomplete_input: false,
                })
            }
        });

        methods.add_method_mut(
            "get_color_at",
            |_, this, (layer, x, y): (usize, i32, i32)| {
                if layer < this.buffer.layers.len() {
                    let ch = this.buffer.layers[layer].get_char((x, y));
                    Ok((ch.attribute.get_foreground(), ch.attribute.get_background()))
                } else {
                    Err(mlua::Error::SyntaxError {
                        message: format!(
                            "Layer {} out of range (0..<{})",
                            layer,
                            this.buffer.layers.len()
                        ),
                        incomplete_input: false,
                    })
                }
            },
        );

        methods.add_method_mut(
            "print",
            |_, this, (layer, mut x, y, str): (usize, i32, i32, String)| {
                if layer < this.buffer.layers.len() {
                    for c in str.chars() {
                        this.buffer.layers[layer]
                            .set_char((x, y), AttributedChar::new(c, this.caret.get_attribute()));
                        x += 1;
                    }
                    Ok(())
                } else {
                    Err(mlua::Error::SyntaxError {
                        message: format!(
                            "Layer {} out of range (0..<{})",
                            layer,
                            this.buffer.layers.len()
                        ),
                        incomplete_input: false,
                    })
                }
            },
        );

        methods.add_method_mut(
            "set_layer_position",
            |_, this, (layer, x, y): (usize, i32, i32)| {
                if layer < this.buffer.layers.len() {
                    this.buffer.layers[layer].set_offset((x, y));
                    Ok(())
                } else {
                    Err(mlua::Error::SyntaxError {
                        message: format!(
                            "Layer {} out of range (0..<{})",
                            layer,
                            this.buffer.layers.len()
                        ),
                        incomplete_input: false,
                    })
                }
            },
        );
        methods.add_method_mut("get_layer_position", |_, this, layer: usize| {
            if layer < this.buffer.layers.len() {
                let pos = this.buffer.layers[layer].get_offset();
                Ok((pos.x, pos.y))
            } else {
                Err(mlua::Error::SyntaxError {
                    message: format!(
                        "Layer {} out of range (0..<{})",
                        layer,
                        this.buffer.layers.len()
                    ),
                    incomplete_input: false,
                })
            }
        });

        methods.add_method_mut(
            "set_layer_visible",
            |_, this, (layer, is_visible): (i32, bool)| {
                let layer = layer as usize;
                if layer < this.buffer.layers.len() {
                    this.buffer.layers[layer].is_visible = is_visible;
                    Ok(())
                } else {
                    Err(mlua::Error::SyntaxError {
                        message: format!(
                            "Layer {} out of range (0..<{})",
                            layer,
                            this.buffer.layers.len()
                        ),
                        incomplete_input: false,
                    })
                }
            },
        );

        methods.add_method_mut("get_layer_visible", |_, this, layer: usize| {
            if layer < this.buffer.layers.len() {
                Ok(this.buffer.layers[layer].is_visible)
            } else {
                Err(mlua::Error::SyntaxError {
                    message: format!(
                        "Layer {} out of range (0..<{})",
                        layer,
                        this.buffer.layers.len()
                    ),
                    incomplete_input: false,
                })
            }
        });

        methods.add_method_mut("clear", |_, this, ()| {
            this.caret = Caret::default();
            this.buffer = Buffer::new(this.buffer.get_size());
            Ok(())
        });
    }
}

const MAX_FRAMES: usize = 4096;
impl Animator {
    pub fn next_frame(&mut self, buffer: &Buffer) -> mlua::Result<()> {
        // Need to limit it a bit to avoid out of memory & slowness
        // Not sure how large the number should be but it's easy to define millions of frames
        if self.frames.len() > MAX_FRAMES {
            return Err(mlua::Error::RuntimeError(
                "Maximum number of frames reached".to_string(),
            ));
        }
        let mut frame = Buffer::new(buffer.get_size());
        frame.layers = buffer.layers.clone();
        frame.terminal_state = buffer.terminal_state.clone();
        frame.palette = buffer.palette.clone();
        frame.layers = buffer.layers.clone();
        frame.clear_font_table();
        for f in buffer.font_iter() {
            frame.set_font(*f.0, f.1.clone());
        }
        self.frames.push(frame);
        Ok(())
    }

    pub fn run(parent: &Option<PathBuf>, txt: &str) -> mlua::Result<Arc<Mutex<Self>>> {
        let lua = Lua::new();
        let globals = lua.globals();
        let animator = Arc::new(Mutex::new(Animator::default()));

        let parent = parent.clone();

        globals
            .set(
                "load_buffer",
                lua.create_function(move |_lua, file: String| {
                    let mut file_name = Path::new(&file).to_path_buf();
                    if file_name.is_relative() {
                        if let Some(parent) = &parent {
                            file_name = parent.join(&file_name);
                        }
                    }

                    if !file_name.exists() {
                        return Err(mlua::Error::RuntimeError(format!(
                            "File not found {}",
                            file
                        )));
                    }

                    if let Ok(buffer) = icy_engine::Buffer::load_buffer(&file_name, true) {
                        mlua::Result::Ok(LuaBuffer {
                            caret: Caret::default(),
                            buffer,
                        })
                    } else {
                        Err(mlua::Error::RuntimeError(format!(
                            "Could not load file {}",
                            file
                        )))
                    }
                })?,
            )
            .unwrap();

        globals
            .set(
                "new_buffer",
                lua.create_function(move |_lua, (width, height): (i32, i32)| {
                    mlua::Result::Ok(LuaBuffer {
                        caret: Caret::default(),
                        buffer: Buffer::create((width, height)),
                    })
                })?,
            )
            .unwrap();

        let a = animator.clone();
        globals
            .set(
                "next_frame",
                lua.create_function_mut(move |lua, buffer: Value<'_>| {
                    if let Value::UserData(data) = &buffer {
                        lua.globals().set("cur_frame", a.lock().frames.len() + 1)?;
                        a.lock().next_frame(&data.borrow::<LuaBuffer>()?.buffer)
                    } else {
                        Err(mlua::Error::RuntimeError(format!(
                            "UserData parameter required, got: {:?}",
                            buffer
                        )))
                    }
                })?,
            )
            .unwrap();
        globals.set("cur_frame", 1)?;

        lua.load(txt).exec()?;
        Ok(animator)
    }
}
