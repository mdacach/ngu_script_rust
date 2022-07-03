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
/// Position{ x: 0, y: 0 } represents the upper left corner.
/// x increases from left to right, and y increases from top to bottom.
pub struct Position {
    pub x: u16,
    pub y: u16,
}

/// This way we can code in terms of normal Positions and have the script
/// convert them to InGamePositions for us.
impl From<Position> for InGamePosition {
    fn from(pos: Position) -> Self {
        let Position { x, y } = pos;
        let Position {
            x: corner_x,
            y: corner_y,
        } = *CORNER;

        InGamePosition {
            x: x + corner_x,
            y: y + corner_y,
        }
    }
}

/// Represents in-game coordinates.
/// InGamePosition{ x: 0, y: 0 } represents the GAME's upper left corner.
/// Depending on where the game's window is located in your screen, Position and InGamePosition will differ.
///
/// Example:
/// Position{ x: 50, y: 50 } represents coordinates on your screen. The game may not even be in that area.
/// InGamePosition{ x: 50, y: 50 } represents coordinates inside the game, close to the upper left corner.
/// In reality, InGamePosition{ x: 50, y: 50 } could be a Position{ x: 550, y: 800 } (if the game corner is
/// on { x: 500, y: 750 }).
pub struct InGamePosition {
    pub x: u16,
    pub y: u16,
}

lazy_static! {
    /// Position of game's upper left corner, at the start of the script.
    pub static ref CORNER: Position = {
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
fn find_game_corner(screenshot: &GrayImage) -> Position {
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

    Position {
        x: x as u16,
        y: y as u16,
    }
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
