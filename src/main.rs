use std::{thread, time};

use image::{open, GenericImage, GrayImage, Luma};
use imageproc::definitions::Image;
use imageproc::map::map_colors;
use imageproc::template_matching::{find_extremes, match_template, MatchTemplateMethod};
use lazy_static::lazy_static;
use rdev::{simulate, Button, EventType, Key, SimulateError};
use screenshots::Screen;

lazy_static! {
    static ref CORNER: (u32, u32) = {
    let left_monitor = Screen::from_point(100, 100).expect("Could not find display screen");
    let screenshot = left_monitor.capture().expect("Could not screenshot");
    // TODO: avoid the extra write to file, we already have the image here
    std::fs::write("images/screenshot.png", screenshot.buffer())
        .expect("Could not save screenshot");

    let screenshot = open("images/screenshot.png")
        .expect("Could not open previous screenshot")
        .to_luma8();

    find_game_corner(&screenshot)
    };
}

fn main() {
    mouse_move(*CORNER);

    for id in 0..24 {
        Inventory::move_to_slot(id as u32);
        thread::sleep(time::Duration::from_millis(300));
    }
}

fn mouse_move((x, y): (u32, u32)) {
    let x = x + CORNER.0;
    let y = y + CORNER.1;
    send(&EventType::MouseMove {
        x: x.into(),
        y: y.into(),
    });
}

fn send(event_type: &EventType) {
    let delay = time::Duration::from_millis(20);
    match simulate(event_type) {
        Ok(()) => (),
        Err(SimulateError) => {
            println!("We could not send {:?}", event_type);
        }
    }
    // Let ths OS catchup (at least MacOS)
    thread::sleep(delay);
}

/// Returns the coordinates of the game upper-left corner.
///
/// The game is windowed and can be moved around the screen, so we do not
/// want to hard-code any coordinates here.
fn find_game_corner(screenshot: &GrayImage) -> (u32, u32) {
    let corner_image = open("images/corner_game.png")
        .expect("Could not open cropped image!")
        .to_luma8();

    let method = MatchTemplateMethod::CrossCorrelationNormalized;
    let result = match_template(screenshot, &corner_image, method);
    let result = convert_to_gray_image(&result);
    let result = {
        let mut r = GrayImage::new(screenshot.width(), screenshot.height());
        r.copy_from(&result, corner_image.width() / 2, corner_image.height() / 2)
            .unwrap();
        r
    };
    result
        .save("images/debug_result_matching.png")
        .expect("Could not save image");

    let extremes = find_extremes(&result);
    println!("Max match found on {:?}", extremes.max_value_location);

    // This matches the center of the image
    // So to find the pixel-point corner, we need to subtract the dimensions
    let (mut x, mut y) = extremes.max_value_location;
    x = x.saturating_sub(corner_image.width() / 2);
    y = y.saturating_sub(corner_image.height() / 2);

    (x, y)
}

/// Convert an f32-valued image to a 8 bit depth, covering the whole
/// available intensity range.
fn convert_to_gray_image(image: &Image<Luma<f32>>) -> GrayImage {
    let mut lo = f32::INFINITY;
    let mut hi = f32::NEG_INFINITY;

    for p in image.iter() {
        lo = if *p < lo { *p } else { lo };
        hi = if *p > hi { *p } else { hi };
    }

    let range = hi - lo;
    let scale = |x| (255.0 * (x - lo) / range) as u8;
    map_colors(image, |p| Luma([scale(p[0])]))
}

struct Inventory;

impl Inventory {
    const SLOT_FIRST: (u32, u32) = (470, 440);
    const SLOT_SIZE: (u32, u32) = (66, 67);
    const SLOTS_PER_ROW: u32 = 12;

    fn move_to_slot(id: u32) {
        let (mut x, mut y) = Self::SLOT_FIRST;
        // Rows wrap around after some slots
        let move_right = id % Self::SLOTS_PER_ROW;
        let move_down = id / Self::SLOTS_PER_ROW;
        x += move_right * Self::SLOT_SIZE.0;
        y += move_down * Self::SLOT_SIZE.1;
        mouse_move((x, y));
    }
}
