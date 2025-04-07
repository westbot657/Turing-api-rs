use std::ffi::{c_char, CString};

#[repr(C)]
pub struct _ColorNote {
    ptr: *mut c_char
}


extern "C" {
    fn _create_color_note(beat: f32) -> _ColorNote;

    fn _beatmap_add_color_note(note: _ColorNote);

    fn _log(message: *const c_char);
}


// User facing definitions:

pub struct Log {}
impl Log {
    pub fn info(message: &str) {
        let s = format!("info: {}", message);
        let c = CString::new(s).unwrap();
        unsafe { _log(c.as_ptr()) };
    }
}


/// Represents a Color Note in the game
pub struct ColorNote {
    _note: _ColorNote
}

/// Holds all data relevant to the beatmap, the environment, and anything else
pub struct Beatmap {
}


/// function to add a color note to beat saber, and apply modifiers from other mods
pub fn create_note(beat: f32) -> ColorNote {
    unsafe { ColorNote { _note: _create_color_note(beat) } }
}

impl Beatmap {
    /// Adds a ColorNote to the playing beatmap
    pub fn add_color_note(note: ColorNote) {
        unsafe {
            _beatmap_add_color_note(note._note);
        }
    }
}
