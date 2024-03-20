use godot::prelude::*;
use godot::engine::Node2D;


#[derive(GodotClass)]
#[class(base=Node2D)]
struct RustNode {
    speed: f64,
    angular_speed: f64,

    base: Base<Node2D>
}


#[godot_api]
impl INode2D for RustNode {
    fn init(base: Base<Node2D>) -> Self {
        godot_print!("Hello, World!");

        Self {
            speed: 400.0,
            angular_speed: std::f64::consts::PI,
            base,
        }
    }
}
