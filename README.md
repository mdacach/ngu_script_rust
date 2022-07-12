## NGU Script
This is a learning project in my Rust journey. 

The objective of this script is to automate the playing of an incremental game called [NGU](https://store.steampowered.com/app/1147690/NGU_IDLE/).

NGU is a complex game with many unwrapping features and requires user input to play optimally. 
Note that this is still under active development and is not meant to be used by the public.

### Code and Feature Highlights
* [Script finds the game anywhere on display with template matching](#script-finds-the-game-anywhere-on-display-with-template-matching)
* [Rust Type System prevents clicking on wrong coordinates](#rust-type-system-prevents-clicking-on-wrong-coordinates)
* [Check for ready skills with image processing](#check-for-ready-skills-with-image-processing)
* [Check for Boss encounters with image processing](#check-for-boss-encounters-with-image-processing)
* [User input termination](#user-input-termination)

#### Script finds the game anywhere on display with template matching
https://user-images.githubusercontent.com/43617028/178321227-21eb6cda-5275-4d83-b1a0-8b0c262a6ee1.mp4

The script takes a screenshot of the display and searches for the upper left corner image

![](https://github.com/mdacach/ngu_script_rust/blob/main/images/corner_game.png)

on the screen using [template matching](https://docs.rs/imageproc/0.19.2/imageproc/template_matching/index.html). It then stores the coordinates into CORNER constant, which is then used for clicking on in-game items.

#### Rust Type System prevents clicking on wrong coordinates
All click-related functions receive a [specific type](https://github.com/mdacach/ngu_script_rust/blob/26676a075ffe6fc801af4ae4254e59baa396fbeb/src/coords.rs#L23-L51), which represents in-game coordinates. The only way to create this type is by passing absolute coordinates, which are then
transformed into in-game ones (using the `CORNER` constant above, which was defined on script startup).

#### Check for ready skills with image processing
The script will give preference to the strongest adventure skills. It checks to see which skills are on cooldown (by looking ta pre-determined pixels and colors).


https://user-images.githubusercontent.com/43617028/178347357-12f27e5e-7a73-4277-8ae8-65672fded5e3.mp4

After using a skill, such as "Ultimate Buff," the script will not attempt to do so again while it is on cooldown. It checks the corresponding pixel, sees that the color is "darker-blue," and thus knows it is on cooldown.

#### Check for Boss encounters with image processing


https://user-images.githubusercontent.com/43617028/178348007-c4abab08-f7b9-42bb-ac33-63f2997a753c.mp4

Boss enemies have a yellow crown beside their names. By checking if it is present, the script can choose to only kill bosses. (Bosses are the only enemies which drop **items**, skipping regular enemies is more efficient for item farming).

#### User input termination
As the script takes control of the mouse and keyboard, trying to stop it manually can be a pain. To solve this problem, the script is always listening for keyboard events: As soon as it sees a "z" pressed, it terminates.
To handle the above, a new thread is spawned to work on the actual scripting (e.g., killing enemies), and the main thread keeps listening for keyboard events.
Related, the user may terminate the script while it is in the middle of some action (e.g., pressing "y" for an "Ultimate Attack"). In that case, after termination, the "y" would STILL keep being pressed and would never be released. To solve this potential case, the script tracks all of the current pressed (and not released) inputs and will [release all of them](https://github.com/mdacach/ngu_script_rust/blob/26676a075ffe6fc801af4ae4254e59baa396fbeb/src/input.rs#L81) before terminating.

##### Crates used:
* [screenshots](https://crates.io/crates/screenshots)
* [imageproc](https://crates.io/crates/imageproc)
* [image](https://crates.io/crates/image)
* [rdev](https://crates.io/crates/rdev)
* [lazy_static](https://crates.io/crates/lazy_static)
* [scrap](https://crates.io/crates/scrap)

##### Dependencies
`xorg-dev`

`libdbus-1-dev`

`pkg-config`

##### Related:
- During development, I've found an [issue with one crate](https://github.com/nashaofu/display-info/issues/1)
- And asked for a [feature request](https://github.com/nashaofu/screenshots-rs/issues/6) too


##### References:
- Some years ago, I wrote a [similar script](https://github.com/mdacach/ngu_script_python) with Python.
- Back then, I had the help of a discord user called Satyric, which also wrote a (much better) [script](https://github.com/kujan/NGU-scripts).
