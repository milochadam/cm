use super::*;
use ncurses::*;

#[derive(PartialEq)]
pub enum StringListState {
    Navigate,
    Editing { new: bool, prev_cursor_y: usize },
}

pub struct StringList {
    pub state: StringListState,
    pub list: ItemList,
    pub edit_field: EditField,
}

impl StringList {
    pub fn new() -> Self {
        Self {
            state: StringListState::Navigate,
            list: ItemList::new(),
            edit_field: EditField::new(),
        }
    }

    pub fn current_item(&self) -> Option<&str> {
        self.list.current_item()
    }

    pub fn render(&self, rect: Rect, focused: bool, global: &mut Global) {
        self.list.render(rect, focused);
        if let StringListState::Editing { .. } = self.state {
            let row = self.list.current_row(rect);
            self.edit_field.render(row);
            global.cursor_y = row.y as i32;
            global.cursor_x = (row.x + self.edit_field.cursor_x % row.w) as i32;
        }
    }

    pub fn duplicate_after(&mut self) {
        if let StringListState::Navigate = self.state {
            self.list.duplicate_after();
        }
    }

    pub fn duplicate_before(&mut self) {
        if let StringListState::Navigate = self.state {
            self.list.duplicate_before();
        }
    }

    pub fn insert_after(&mut self, global: &mut Global) {
        if let StringListState::Navigate = self.state {
            self.state = StringListState::Editing {
                new: true,
                prev_cursor_y: self.list.cursor_y,
            };
            self.list.insert_after_current(String::new());
            self.edit_field.buffer.clear();
            self.edit_field.cursor_x = 0;
            global.cursor_visible = true;
        }
    }

    pub fn insert_before(&mut self, global: &mut Global) {
        if let StringListState::Navigate = self.state {
            self.state = StringListState::Editing {
                new: true,
                prev_cursor_y: self.list.cursor_y,
            };
            self.list.insert_before_current(String::new());
            self.edit_field.buffer.clear();
            self.edit_field.cursor_x = 0;
            global.cursor_visible = true;
        }
    }

    pub fn start_editing(&mut self, global: &mut Global) {
        if let StringListState::Navigate = self.state {
            if let Some(item) = self.list.current_item() {
                self.edit_field.cursor_x = item.len();
                self.edit_field.buffer = String::from(item);
                self.state = StringListState::Editing {
                    new: false,
                    prev_cursor_y: self.list.cursor_y,
                };
                global.cursor_visible = true;
            }
        }
    }

    pub fn accept_editing(&mut self, global: &mut Global) {
        if let StringListState::Editing { .. } = self.state {
            self.state = StringListState::Navigate;
            self.list.items[self.list.cursor_y] = self.edit_field.buffer.clone();
            global.cursor_visible = false;
        }
    }

    pub fn cancel_editing(&mut self, global: &mut Global) {
        if let StringListState::Editing { new, prev_cursor_y } = self.state {
            self.state = StringListState::Navigate;
            if new {
                self.list.delete_current();
                self.list.cursor_y = prev_cursor_y
            }
            global.cursor_visible = false;
        }
    }

    pub fn handle_key(&mut self, key_stroke: KeyStroke, global: &mut Global) {
        match self.state {
            StringListState::Navigate => {
                if !global.handle_key(key_stroke) {
                    match key_stroke {
                        KeyStroke {
                            key: KEY_I,
                            alt: true,
                        } => {
                            self.duplicate_after();
                        }
                        KeyStroke {
                            key: KEY_SHIFT_I,
                            alt: true,
                        } => {
                            self.duplicate_before();
                        }
                        KeyStroke {
                            key: KEY_I,
                            alt: false,
                        } => {
                            self.insert_after(global);
                        }
                        KeyStroke {
                            key: KEY_SHIFT_I,
                            alt: false,
                        } => {
                            self.insert_before(global);
                        }
                        KeyStroke { key: KEY_F2, .. } => {
                            self.start_editing(global)
                        }
                        key_stroke => self.list.handle_key(key_stroke),
                    }
                }
            }
            StringListState::Editing { .. } => match key_stroke {
                KeyStroke {
                    key: KEY_RETURN, ..
                } => {
                    self.accept_editing(global);
                }
                KeyStroke {
                    key: KEY_ESCAPE, ..
                } => {
                    self.cancel_editing(global);
                }
                key_stroke => self.edit_field.handle_key(key_stroke),
            },
        }
    }
}
