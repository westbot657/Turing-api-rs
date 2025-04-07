use std::ffi::{c_char, CString};

macro_rules! typed_ptr {
    ( $name:ident ) => {
        #[repr(C)]
        pub struct $name {
            ptr: *mut c_char
        }
    };
}

typed_ptr!(_ColorNote);
typed_ptr!(_BombNote);
typed_ptr!(_Arc);
typed_ptr!(_Wall);
typed_ptr!(_Saber);
typed_ptr!(_Player);

extern "C" {
    fn _create_color_note(beat: f32) -> _ColorNote;
    fn _create_bomb_note(beat: f32) -> _BombNote;

    fn _beatmap_add_color_note(note: _ColorNote);
    fn _beatmap_add_bomb_note(bomb: _BombNote);

    fn _log(message: *const c_char);
}

macro_rules! cstr {
    ( $str:expr ) => {
        CString::new($str).unwrap()
    };
}

// User facing definitions:

pub struct Log {}
impl Log {
    pub fn info(message: &str) {
        let s = format!("info: {}", message);
        let c = cstr!(s);
        unsafe { _log(c.as_ptr()) };
    }

    pub fn warning(message: &str) {
        let s = format!("warning: {}", message);
        let c = cstr!(s);
        unsafe { _log(c.as_ptr()) };
    }

    pub fn error(message: &str) {
        let s = format!("error: {}", message);
        let c = cstr!(s);
        unsafe { _log(c.as_ptr()) };
    }

    pub fn debug(message: &str) {
        let s = format!("debug: {}", message);
        let c = cstr!(s);
        unsafe { _log(c.as_ptr()) };
    }

}


/// Represents a Color Note in the game
pub struct ColorNote {
    _note: _ColorNote
}

/// Represents a Bomb Note in the game
pub struct BombNote {
    _bomb: _BombNote
}

/// Holds all data relevant to the beatmap, the environment, and anything else
pub struct Beatmap {
}


/// function to add a color note to beat saber, and apply modifiers from other mods
pub fn create_note(beat: f32) -> ColorNote {
    unsafe { ColorNote { _note: _create_color_note(beat) } }
}

/// function to add a bomb note to beat saber, and apply modifiers from other mods
pub fn create_bomb(beat: f32) -> BombNote {
    unsafe { BombNote { _bomb: _create_bomb_note(beat) } }
}

impl Beatmap {
    /// Adds a ColorNote to the playing beatmap
    pub fn add_color_note(note: ColorNote) {
        unsafe {
            _beatmap_add_color_note(note._note);
        }
    }

    pub fn add_bomb_note(bomb: BombNote) {
        unsafe {
            _beatmap_add_bomb_note(bomb._bomb);
        }
    }

}
