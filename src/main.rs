mod cube;
mod point3d;

use cube::Cube;

fn main() {
    let mut cube = Cube::new();
    cube.init();
    cube.set_fullscreen();

    while !cube.exit {
        cube.print_2d_projection();
        cube.handle_input();
        cube.clear_screen();
    }

    cube.exit();
}
