use std::alloc::Layout;
use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::{c_char, c_void, CString};
use once_cell::unsync::Lazy;
use paste::paste;

macro_rules! typed_ptr {
    ( $name:ident ) => {
        type $name = i32;
    };
}

typed_ptr!(_Color);
typed_ptr!(_ColorNote);
typed_ptr!(_BombNote);
typed_ptr!(_ChainHeadNote);
typed_ptr!(_ChainLinkNote);
typed_ptr!(_ChainNote);
typed_ptr!(_Arc);
typed_ptr!(_Wall);
typed_ptr!(_Saber);
// typed_ptr!(_Player);
typed_ptr!(_Vec2);
typed_ptr!(_Vec3);
typed_ptr!(_Vec4);
typed_ptr!(_Quat);

macro_rules! color_methods {
    ( $tp:ty, $name:tt ) => {
        paste! {
            extern "C" {
                fn [<_ $name _set_color>]($name: $tp, color: _Color);
                fn [<_ $name _get_color>]($name: $tp) -> _Color;
            }
        }
    }
}

macro_rules! position_methods {
    ( $tp:ty, $name:tt ) => {
        paste! {
            extern "C" {
                fn [<_ $name _set_position>]($name: $tp, pos: _Vec3);
                fn [<_ $name _get_position>]($name: $tp) -> _Vec3;
                fn [<_ $name _set_orientation>]($name: $tp, orientation: _Quat);
                fn [<_ $name _get_orientation>]($name: $tp) -> _Quat;
            }
        }
    }
}

macro_rules! gameplay_object_methods {
    ( $tp:ty, $name:tt ) => {
        paste! {
            extern "C" {
                fn [<_create_ $name>](beat: f32) -> $tp;
                fn [<_beatmap_add_ $name>]($name: $tp);
                fn [<_beatmap_remove_ $name>]($name: $tp);
                fn [<_beatmap_get_ $name _at_beat>](beat: f32) -> $tp;
            }
        }
        position_methods! { $tp, $name }
        color_methods! { $tp, $name }
    };
}

macro_rules! attrs {
    ( $tp:ty, $name:tt, $( $attr:ident: $attr_tp:ty ),* $(,)? ) => {
        extern "C" {
            paste! {
                $(
                fn [<_ $name _get_ $attr>]($name: [<_ $tp>]) -> $attr_tp;
                fn [<_ $name _set_ $attr>]($name: [<_ $tp>], $attr: $attr_tp);
                )*
            }
        }

        impl $tp {
            $(

            paste! {
                pub fn [<get_ $attr>](&self) -> $attr_tp {
                    unsafe { [<_ $name _get_ $attr>](self._inner) }
                }
                pub fn [<set_ $attr>](&self, $attr: $attr_tp) {
                    unsafe { [<_ $name _set_ $attr>](self._inner, $attr) }
                }
            }

            )*
        }
    };
}

gameplay_object_methods! { _ColorNote, color_note }
gameplay_object_methods! { _BombNote, bomb_note }
gameplay_object_methods! { _Arc, arc }
gameplay_object_methods! { _Wall, wall }
gameplay_object_methods! { _ChainHeadNote, chain_head_note }
gameplay_object_methods! { _ChainLinkNote, chain_link_note }
gameplay_object_methods! { _ChainNote, chain_note }

color_methods! { _Saber, saber }

/// Required for the wasm host to be able to reliably allocate memory for any language
extern "C" fn _malloc(size: i32) -> *const c_void {
    unsafe {
         std::alloc::alloc(Layout::from_size_align(size as usize, 8).unwrap()) as *const c_void
    }
}

/// Required for the wasm host to be able to reliably deallocate memory for any language
extern "C" fn _free(ptr: *mut c_void, size: i32) {
    unsafe {
        std::alloc::dealloc(ptr as *mut u8, Layout::from_size_align(size as usize, 8).unwrap());
    }
}

extern "C" {

    fn _get_left_saber() -> _Saber;
    fn _get_right_saber() -> _Saber;

    fn _log(message: *const c_char);

    // drops a host-managed object such as color notes, vectors, walls, etc
    fn _drop_reference(ptr: *mut c_char);

    fn _vec2_from_xy(x: f32, y: f32) -> _Vec2;
    fn _vec3_from_xyz(x: f32, y: f32, z: f32) -> _Vec3;
    fn _vec4_from_xyzw(x: f32, y: f32, z: f32, w: f32) -> _Vec4;
    fn _quat_from_xyzw(x: f32, y: f32, z: f32, w: f32) -> _Quat;

    fn _color_set_rgb(color: _Color, r: f32, g: f32, b: f32);
    fn _color_set_rgba(color: _Color, r: f32, g: f32, b: f32, a: f32);

    fn _data_contains_persistent_i32(key: *const c_char) -> bool;
    fn _data_contains_persistent_f32(key: *const c_char) -> bool;
    fn _data_contains_persistent_str(key: *const c_char) -> bool;

    fn _data_store_persistent_i32(key: *const c_char, value: i32);
    fn _data_store_persistent_f32(key: *const c_char, value: f32);
    fn _data_store_persistent_str(key: *const c_char, value: *const c_char);

    fn _data_access_persistent_i32(key: *const c_char) -> i32;
    fn _data_access_persistent_f32(key: *const c_char) -> f32;
    fn _data_access_persistent_str(key: *const c_char) -> *const c_char;

    fn _data_remove_persistent_i32(key: *const c_char);
    fn _data_remove_persistent_f32(key: *const c_char);
    fn _data_remove_persistent_str(key: *const c_char);


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

macro_rules! wrapped {
    ( $name:tt ) => {
        pub struct $name {
            _inner: paste! { [<_ $name>] }
        }
    };
}

macro_rules! instantiable_obj {
    ( $ty:tt, $name:tt ) => {
        wrapped! { $ty }
        paste! {

            pub fn [<create_ $name>](beat: f32) -> $ty {
                unsafe { $ty { _inner: [<_create_ $name>](beat) } }
            }

            impl Beatmap {
                pub fn [<add_ $name>]($name: $ty) {
                    unsafe {
                        [<_beatmap_add_ $name>]($name._inner);
                    }
                }

                pub fn [<remove_ $name>]($name: $ty) {
                    unsafe {
                        [<_beatmap_remove_ $name>]($name._inner);
                    }
                }

                pub fn [<get_ $name _at_beat>](beat: f32) {
                    unsafe {
                        [<_beatmap_get_ $name _at_beat>](beat);
                    }
                }

            }
        }
    };
}

pub struct Beatmap;

thread_local! {
    static GLOBAL_MAP: Lazy<RefCell<HashMap<String, Box<dyn Any>>>> = Lazy::new(|| {
        RefCell::new(HashMap::new())
    });
}

pub struct Data;

impl Data {

    pub fn set_temp_value<T: Clone + 'static>(key: &str, v: T) {
        GLOBAL_MAP.with(|map| {
            map.borrow_mut().insert(key.to_string(), Box::new(v));
        })
    }

    pub fn get_temp_value<T: Clone + 'static>(key: &str) -> Option<T> {
        GLOBAL_MAP.with(|map| {
            map.borrow().get(key).and_then(|v| v.downcast_ref::<T>()).cloned()
        })
    }

    pub fn remove_temp_value(key: &str) {
        GLOBAL_MAP.with(|map| {
            map.borrow_mut().remove(key);
        })
    }

    pub fn get_persistent_i32(key: &str) -> Option<i32> {
        let c_str = CString::new(key).unwrap();
        unsafe {
            if _data_contains_persistent_i32(c_str.as_ptr()) {
                Some(_data_access_persistent_i32(c_str.as_ptr()))
            } else {
                None
            }
        }
    }

    pub fn get_persistent_f32(key: &str) -> Option<f32> {
        let c_str = CString::new(key).unwrap();
        unsafe {
            if _data_contains_persistent_f32(c_str.as_ptr()) {
                Some(_data_access_persistent_f32(c_str.as_ptr()))
            } else {
                None
            }
        }
    }

    pub fn get_persistent_str(key: &str) -> Option<String> {
        let c_str = CString::new(key).unwrap();
        unsafe {
            if _data_contains_persistent_str(c_str.as_ptr()) {
                let ptr = _data_access_persistent_str(c_str.as_ptr());
                let cstr = CString::from_raw(ptr as *mut c_char);
                Some(cstr.to_string_lossy().to_string())
            } else {
                None
            }
        }
    }

}

instantiable_obj! { ColorNote, color_note }
instantiable_obj! { BombNote, bomb_note }
instantiable_obj! { ChainHeadNote, chain_head_note }
instantiable_obj! { ChainLinkNote, chain_link_note }
instantiable_obj! { ChainNote, chain_note }
instantiable_obj! { Arc, arc }
instantiable_obj! { Wall, wall }

wrapped! { Vec2 }
wrapped! { Vec3 }
wrapped! { Vec4 }
wrapped! { Quat }
wrapped! { Color }

attrs! { Vec2, vec2, x: f32, y: f32 }
attrs! { Vec3, vec3, x: f32, y: f32, z: f32 }
attrs! { Vec4, vec4, x: f32, y: f32, z: f32, w: f32 }
attrs! { Quat, quat, x: f32, y: f32, z: f32, w: f32 }
attrs! { Color, color, r: f32, g: f32, b: f32, a: f32 }

pub trait UnityConvertible {
    type UnityType;
    fn to_unity_type(self) -> Self::UnityType;
    fn from_unity_type(t: Self::UnityType) -> Self;
}

impl UnityConvertible for glam::Vec2 {
    type UnityType = Vec2;
    fn to_unity_type(self) -> Self::UnityType {
        Vec2 { _inner: unsafe { _vec2_from_xy(self.x, self.y) } }
    }
    fn from_unity_type(t: Self::UnityType) -> Self {
        glam::Vec2::new(t.get_x(), t.get_y())
    }
}

impl UnityConvertible for glam::Vec3 {
    type UnityType = Vec3;
    fn to_unity_type(self) -> Self::UnityType {
        Vec3 { _inner: unsafe { _vec3_from_xyz(self.x, self.y, self.z) } }
    }
    fn from_unity_type(t: Self::UnityType) -> Self {
        glam::Vec3::new(t.get_x(), t.get_y(), t.get_z())
    }
}

impl UnityConvertible for glam::Vec4 {
    type UnityType = Vec4;
    fn to_unity_type(self) -> Self::UnityType {
        Vec4 { _inner: unsafe { _vec4_from_xyzw(self.x, self.y, self.z, self.w) } }
    }
    fn from_unity_type(t: Self::UnityType) -> Self {
        glam::Vec4::new(t.get_x(), t.get_y(), t.get_z(), t.get_w())
    }
}

impl UnityConvertible for glam::Quat {
    type UnityType = Quat;
    fn to_unity_type(self) -> Self::UnityType {
        Quat { _inner: unsafe { _quat_from_xyzw(self.x, self.y, self.z, self.w) } }
    }
    fn from_unity_type(t: Self::UnityType) -> Self {
        glam::Quat::from_xyzw(t.get_x(), t.get_y(), t.get_z(), t.get_w())
    }
}


impl Color {
    pub fn set_rgb(&self, r: f32, g: f32, b: f32) {
        unsafe { _color_set_rgb(self._inner, r, g, b) };
    }

    pub fn set_rgba(&self, r: f32, g: f32, b: f32, a: f32) {
        unsafe { _color_set_rgba(self._inner, r, g, b, a) };
    }
}

