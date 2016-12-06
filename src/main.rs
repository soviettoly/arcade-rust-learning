extern crate sdl2;

mod phi;
mod views;

use phi::{Events, Phi, View, ViewAction};

fn main() {
    ::phi::spawn("ArcadeRS Shooter", |phi| {
        Box::new(::views::ShipView::new(phi))
    });

}
