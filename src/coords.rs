use image::{open, GenericImage, GrayImage, Luma};
use imageproc::definitions::Image;
use imageproc::map::map_colors;
use imageproc::template_matching::{find_extremes, match_template, MatchTemplateMethod};
use lazy_static::lazy_static;
use screenshots::Screen;

/// Represents the size of a rectangle.
pub struct Size {
    pub width: u16,
    pub height: u16,
}

/// Represents absolute coordinates on a screen.
/// AbsolutePosition{ x: 0, y: 0 } represents the upper left corner.
/// x increases from left to right, and y increases from top to bottom.
#[derive(Copy, Clone)]
pub struct AbsolutePosition {
    pub x: u16,
    pub y: u16,
}

/// Represents clickable coordinates in game.
/// GameAwarePosition{ x: 0, y: 0 } represents the GAME's upper left corner.
/// Depending on where the game's window is located in your screen, AbsolutePosition and GameAwarePosition will differ.
///
/// Example:
/// AbsolutePosition{ x: 50, y: 50 } represents coordinates on your screen, close to your display upper left corner.
/// The game may not even be in that area.
/// GameAwarePosition{ x: 50, y: 50 } represents coordinates inside the game, close to the upper left corner.
/// GameAwarePosition is calculated from a given AbsolutePosition. (We find the game corner and update the position
/// accordingly so that it is inside the game's window).
#[derive(Copy, Clone, Debug)]
// Non-exhaustive makes the struct only constructable from `from_coords` below.
#[non_exhaustive] // Reference: https://stackoverflow.com/a/70965787
pub struct GameAwarePosition {
    pub x: u16,
    pub y: u16,
}

impl GameAwarePosition {
    pub fn from_coords(x: u16, y: u16) -> GameAwarePosition {
        let AbsolutePosition {
            x: corner_x,
            y: corner_y,
        } = *CORNER;

        GameAwarePosition {
            x: x + corner_x,
            y: y + corner_y,
        }
    }
}

#[non_exhaustive]
pub struct GameAwareRectangle {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl GameAwareRectangle {
    pub fn from_coords(x: u16, y: u16, width: u16, height: u16) -> Self {
        let AbsolutePosition {
            x: corner_x,
            y: corner_y,
        } = *CORNER;
        Self {
            x: x + corner_x,
            y: y + corner_y,
            width,
            height,
        }
    }
}

lazy_static! {
    /// Absolute position of game's upper left corner, at the start of the script.
    pub static ref CORNER: AbsolutePosition = {
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

/// Returns the coordinates of the game upper-left corner.
///
/// The game is windowed and can be moved around the screen, so we do not
/// want to hard-code any coordinates here.
/// Reference: https://github.com/image-rs/imageproc/blob/master/examples/template_matching.rs
fn find_game_corner(screenshot: &GrayImage) -> AbsolutePosition {
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

    AbsolutePosition {
        x: x as u16,
        y: y as u16,
    }
}

/// Convert an f32-valued image to a 8 bit depth, covering the whole
/// available intensity range.
///
/// Reference: https://github.com/image-rs/imageproc/blob/master/examples/template_matching.rs
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
