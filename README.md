# Turing API

The rust Turing scripting API for Beat Saber


This API exposes methods defined by the Turing mod for Beat Saber map scripting

This is a work-in-progress and will not be usable until the Turing mod is released (probably will be a while)



# Traits

### UnityConvertible
```rust
pub trait UnityConvertible {
    type UnityType;
    fn to_unity_type(self) -> Self::UnityType;
    fn from_unity_type(t: Self::UnityType) -> Self;
}
```


# Structs

## Static types

---
### Beatmap

```rust
impl Beatmap {
  
  pub fn add_color_note(note: ColorNote) { ... }
  
  pub fn add_bomb_note(note: ColorNote) { ... }
  
  pub fn add_chain_head_note(note: ColorNote) { ... }
  
  pub fn add_chain_link_note(note: ColorNote) { ... }
  
  pub fn add_chain_note(note: ColorNote) { ... }
  
  pub fn add_arc(note: ColorNote) { ... }
  
  pub fn add_wall(note: ColorNote) { ... }
  
}
```

---
### Log
```rust
impl Log {
  
  pub fn info(message: &str) {...}

  pub fn warning(message: &str) {...}

  pub fn error(message: &str) {...}

  pub fn debug(message: &str) {...}
  
}
```

## Instanced types

---
### Color
```rust
impl Color {
  pub fn set_rgb(&mut self, r: f32, g: f32, b: f32) {...}
  pub fn set_rgba(&mut self, r: f32, g: f32, b: f32, a: f32) {...}
}
```

---
### ColorNote

---
### BombNote

---
### ChainHeadNote

---
### ChainLinkNote

---
### ChainNote

---
### Arc

---
### Wall

---
### Saber

---
### Vec2
```rust
impl Vec2 {
  pub fn get_x(&self) -> f32 { ... }
  pub fn get_y(&self) -> f32 { ... }
  
  pub fn set_x(&self, x: f32) { ... }
  pub fn set_y(&self, y: f32) { ... }
}
```

```rust
impl UnityConvertible for glam::Vec2 {
  type UnityType = Vec2;
  // ...
}
```

---
### Vec3
```rust
impl Vec3 {
  pub fn get_x(&self) -> f32 { ... }
  pub fn get_y(&self) -> f32 { ... }
  pub fn get_z(&self) -> f32 { ... }
  
  pub fn set_x(&self, x: f32) { ... }
  pub fn set_y(&self, y: f32) { ... }
  pub fn set_z(&self, z: f32) { ... }
}
```

```rust
impl UnityConvertible for glam::Vec3 {
  type UnityType = Vec3;
  // ...
}
```


---
### Vec4
```rust
impl Vec4 {
  pub fn get_x(&self) -> f32 { ... }
  pub fn get_y(&self) -> f32 { ... }
  pub fn get_z(&self) -> f32 { ... }
  pub fn get_w(&self) -> f32 { ... }
  
  pub fn set_x(&self, x: f32) { ... }
  pub fn set_y(&self, y: f32) { ... }
  pub fn set_z(&self, z: f32) { ... }
  pub fn set_w(&self, z: f32) { ... }
}
```

```rust
impl UnityConvertible for glam::Vec4 {
  type UnityType = Vec4;
  // ...
}
```

---
### Quat
```rust
impl Quat {
  pub fn get_x(&self) -> f32 { ... }
  pub fn get_y(&self) -> f32 { ... }
  pub fn get_z(&self) -> f32 { ... }
  pub fn get_w(&self) -> f32 { ... }
  
  pub fn set_x(&self, x: f32) { ... }
  pub fn set_y(&self, y: f32) { ... }
  pub fn set_z(&self, z: f32) { ... }
  pub fn set_w(&self, z: f32) { ... }
}
```

```rust
impl UnityConvertible for glam::Quat {
  type UnityType = Quat;
  // ...
}
```

## Static Functions

```rust
pub fn create_color_note(beat: f32) -> ColorNote {...}
```

```rust
pub fn create_bomb_note(beat: f32) -> BombNote {...}
```

```rust
pub fn create_chain_head_note(beat: f32) -> ChainHeadNote {...}
```

```rust
pub fn create_chain_link_note(beat: f32) -> ChainLinkNote {...}
```

```rust
pub fn create_chain_note(beat: f32) -> ChainNote {...}
```

```rust
pub fn create_arc(beat: f32) -> Arc {...}
```

```rust
pub fn create_wall(beat: f32) -> Wall {...}
```
