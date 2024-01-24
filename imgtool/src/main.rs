use std::env;
use std::path::Path;

fn main() {
    let image_path = env::args().skip(1).next().expect("one argument is missing");
    let path = Path::new(&image_path);
    let img = image::open(path).expect("the path should be valid");
    let rotated = img.rotate90();
    rotated.save(path).expect("the path should be still valid");
}
