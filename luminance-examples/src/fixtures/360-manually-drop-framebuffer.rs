//! <https://github.com/phaazon/luminance-rs/issues/360>
//!
//! When a framebuffer is created, dropped and a new one is created, the new framebuffer doesn’t
//! seem to behave correctly. At the time of #360, it was detected on Windows.
//!
//! Because this example simply creates a framebuffer, drops it and creates another one, nothing is
//! displayed and the window flashes on the screen. Run the application in a tool such as apitrace
//! or renderdoc to analyze it.

use luminance::context::GraphicsContext as _;
use luminance::pixel::{Depth32F, RGBA32F};
use luminance::texture::{Dim2, Sampler};
use luminance_glfw::GlfwSurface;
use luminance_windowing::WindowOpt;

fn main() {
  let mut surface =
    GlfwSurface::new_gl33("fixture-360", WindowOpt::default()).expect("GLFW surface");

  let framebuffer =
    surface.new_framebuffer::<Dim2, RGBA32F, Depth32F>([1024, 1024], 0, Sampler::default());

  std::mem::drop(framebuffer);

  // #360 occurs here after the drop
  let framebuffer =
    surface.new_framebuffer::<Dim2, RGBA32F, Depth32F>([1024, 1024], 0, Sampler::default());
}
