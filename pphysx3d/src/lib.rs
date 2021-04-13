extern crate kiss3d;
use kiss3d::nalgebra as na;

mod collision;
mod force;
mod renderer;
mod scene;
mod shapes;

fn main() {
    let mut window = kiss3d::window::Window::new_with_size("kiss3d: cube", 600, 300);
    let mut cube = window.add_cube(0.4, 0.4, 0.4);

    cube.set_color(1.0, 0.0, 0.0);

    window.set_light(kiss3d::light::Light::StickToCamera);

    let rotation = na::UnitQuaternion::from_axis_angle(&na::Vector3::y_axis(), 0.014);
    while window.render() {
        cube.append_rotation(&rotation);
    }
}
