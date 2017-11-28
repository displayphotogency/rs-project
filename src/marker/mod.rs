pub mod geom;

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Debug)]
pub enum Square {
    LT,
    MT,
    RT,
    LM,
    MM,
    RM,
    LB,
    MB,
    RB,
    IBL,
    ILT,
    IBR,
    IRT,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Debug)]
pub enum Horizontal {
    Left,
    Right,
    Center,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Debug)]
pub enum SpriteType {
    Object,
    Ground { square: Vec<Square> },
}

impl SpriteType {
    pub fn empty_ground() -> SpriteType {
        SpriteType::Ground { square: vec![] }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SpriteData {
    pub on_screen_frame: geom::Rect,
    pub frame: geom::Rect,
    pub markers: SpriteType,
    pub name: String,
    pub index: usize,
}
