use specs::{World, VecStorage, DenseVecStorage, NullStorage};

#[derive(Debug, Component, Copy, Clone)]
#[component(VecStorage)]
pub struct PositionComponent {
    pub x: f32,
    pub y: f32,
}
impl PositionComponent {
    pub fn new(x: f32, y: f32) -> PositionComponent {
        PositionComponent { x, y }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum RenderComponentType {
    Animation {
        id: &'static str,
        frame: usize,
        length: usize,
    },
    Image { id: &'static str },
    Batch { id: &'static str },
}

impl RenderComponentType {
    pub fn set_animation_id(&mut self, new_id: &'static str, new_length: usize) {
        match self {
            &mut RenderComponentType::Animation {
                ref mut id,
                ref mut length,
                ..
            } => {
                *id = new_id;
                *length = new_length;
            }
            _ => (),
        }
    }
}

#[derive(Debug, Component)]
#[component(VecStorage)]
pub struct RenderComponent {
    pub layer: usize,
    pub render_type: RenderComponentType,
}

#[derive(Debug, Component)]
#[component(DenseVecStorage)]
pub struct ControlComponent;

#[derive(Debug, Component, Default)]
#[component(NullStorage)]
pub struct SnapCameraComponent;

#[derive(Debug, Component, Default)]
#[component(NullStorage)]
pub struct ChaseCameraComponent;

#[derive(Debug, Component, Clone, Copy)]
#[component(VecStorage)]
pub enum DirectionComponent {
    Left,
    Right,
}

#[derive(Debug, Component, Clone, Copy)]
#[component(DenseVecStorage)]
pub struct ScaleComponent {
    pub x: f32,
    pub y: f32,
}

impl ScaleComponent {
    pub fn new(x: f32, y: f32) -> Scalable {
        Scalable { x, y }
    }
}

pub fn register_components(world: &mut World) {
    world.register::<PositionComponent>();
    world.register::<RenderComponent>();
    world.register::<ChaseCameraComponent>();
}
