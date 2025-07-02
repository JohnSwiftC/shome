use crate::airplay::airplay_device_flood;

mod airplay;
mod upnp;
mod utils;

fn main() {
    airplay_device_flood("gus", 50);
}
