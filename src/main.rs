#[macro_use]
extern crate log;
extern crate libgelf;

pub fn main() {
    libgelf::init();
    for x in 0..1000 {
        info!("msg-{}", x);
    }
}
