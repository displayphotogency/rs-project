extern crate cpal;
extern crate rodio;

mod resource_manager;
mod gm;

fn main() {
    gm::init_gm();
}
