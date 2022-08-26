use anyhow::Result;
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};

use self::x11::X11Renderer;

pub mod x11;

/// WallpaperRenderer is a trait that
/// enables cross-platform rendering to a wallpaper.
/// Provides a simple lifecycle. wgpu uses the `HasRawWindowHandle`
/// trait in order to use this.
pub trait WallpaperSurface: HasRawWindowHandle + HasRawDisplayHandle {
    fn new() -> Result<Self>
    where
        Self: Sized;
    fn destroy(&self) -> Result<()>;
    fn redraw(&self) -> Result<()>;
    fn dimensions(&self) -> (u16, u16);
}

pub fn new_wallpaper_surface() -> Result<impl WallpaperSurface> {
    X11Renderer::new()
    // cfg_if! {
    //   if #[cfg(windows)] {

    //   } else if #[cfg(linux)] {

    //   }
    // }
}
