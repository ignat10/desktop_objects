mod screen;

use pyo3::prelude::*;
use enigo::{Settings, Enigo, Mouse, Button, Direction, Coordinate};



fn mouse() -> Enigo {
    Enigo::new(&Settings::default()).unwrap()
}


#[pymodule]
mod desktop_objects {
    use super::*;
}


fn click(
    x: impl Into<i32>,
    y: impl Into<i32>
) {
    let mut mouse = mouse();
    mouse.move_mouse(x.into(), y.into(), Coordinate::Abs).unwrap();
    mouse.button(Button::Left, Direction::Click).unwrap();
}


