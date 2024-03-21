use godot::prelude::*;
use godot::engine::Node2D;



#[derive(GodotClass)]
#[class(base=Node2D)]
struct RustNode {
    base: Base<Node2D>
}


#[godot_api]
impl INode2D for RustNode {
    fn init(base: Base<Node2D>) -> Self {

        Self {
            base, 
        }
    }
}
