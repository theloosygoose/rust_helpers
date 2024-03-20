use godot::prelude::*;

struct Helpers;

mod rustnode;

#[gdextension]
unsafe impl ExtensionLibrary for Helpers {}
