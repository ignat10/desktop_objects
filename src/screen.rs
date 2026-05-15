use std::sync::{OnceLock, RwLock};
use screenshots::Screen;
use pixen::Image;
use anyhow::Result;

static SCREEN: OnceLock<Screen> = OnceLock::new();
static SCREENSHOT: RwLock<Option<Image>> = RwLock::new(None);

pub(super) fn capture_full() {
    let mut screenshot_guard = SCREENSHOT.write().unwrap();
    if screenshot_guard.is_none() {
        let screen = get_screen();
        let rgba_img = screen.capture().unwrap();

        let w = rgba_img.width() as usize;
        let h = rgba_img.height() as usize;
        *screenshot_guard = Some(Image::new(rgba_img.into_raw(), w, h, 4usize))
    };
}


pub(super) fn capture_part(
    x: impl Into<i32>,
    y: impl Into<i32>,
    w: impl Into<u32>,
    h: impl Into<u32>) -> Result<Image> {
    let x = x.into();
    let y = y.into();
    let w = w.into();
    let h = h.into();

    let screen = get_screen();

    let rgba = screen.capture_area(
        x.into(),
        y.into(),
        w.into(),
        h.into()
    )?;
    Ok(Image::new(rgba.into_raw(), w as usize, h as usize, 4usize))
}


fn get_screen() -> &'static Screen {
    SCREEN.get_or_init(|| {
        Screen::all().unwrap().into_iter().next().unwrap()
    })
}
