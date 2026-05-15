mod screen;

use std::fs::read_dir;
use std::path::PathBuf;
use std::sync::OnceLock;

use pyo3::prelude::*;
use rayon::prelude::*;
use enigo::{Settings, Enigo, Mouse, Button, Direction, Coordinate};
use screenshots::image;
use pixen::{Image, find_sample};

static IMAGES: OnceLock<PathBuf> = OnceLock::new();

const RGB_CHANNELS: u32 = 3;

fn mouse() -> Enigo {
    Enigo::new(&Settings::default()).unwrap()
}


#[pymodule]
mod desktop_objects {
    use std::collections::HashMap;
    use super::*;

    #[pyfunction]
    fn get_objects(images_dir: PathBuf) -> PyResult<HashMap<String, Object>> {
        IMAGES.set(images_dir).unwrap();

        let mut dict = HashMap::new();
        for entry in read_dir(IMAGES.get().unwrap())? {
            let entry = entry?;
            let name = entry.path().to_str().unwrap().to_string();

            let object = Object::new(name.clone());
            dict.insert(name, object);
        }
        Ok(dict)
    }
}


#[pyclass]
struct Object {
    name: String,
    images: Vec<Image>,
}


#[pymethods]
impl Object {
    fn click(&mut self) -> PyResult<bool> {
        screen::capture_full();

        let guard = screen::SCREENSHOT.read().unwrap();
        let screenshot = guard.as_ref().unwrap();
        Ok(if let Some(coords) = self.par_iter_images().find_map_any(|image| find_sample(screenshot, image)) {
            click(coords.0 as i32, coords.1 as i32);
            screen::reset_screen();
            true
        } else {
            false
        })
    }
}


impl Object {
    fn new(name: String) -> Self {
        Self {
            name,
            images: Vec::new(),
        }
    }

    fn par_iter_images(&mut self) -> impl ParallelIterator<Item = &Image> {
        if self.images.is_empty() {
            let p = self.name.clone();
            self.images = read_dir(IMAGES.get().unwrap().join(p)).unwrap().map(|entry| {
                let path = entry.unwrap().path();
                let rgb = image::open(path).unwrap().into_rgb8();

                let w = rgb.width();
                let h = rgb.height();
                Image::new(rgb.into_raw(), w, h, RGB_CHANNELS)
            }).collect();
        }

        self.images.par_iter()
    }
}
fn click(
    x: impl Into<i32>,
    y: impl Into<i32>
) {
    let mut mouse = mouse();
    mouse.move_mouse(x.into(), y.into(), Coordinate::Abs).unwrap();
    mouse.button(Button::Left, Direction::Click).unwrap();
}


