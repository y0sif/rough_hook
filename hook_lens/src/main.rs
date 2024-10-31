use hook_lens::data;
use hook_lens::inference;
fn main() {
    println!("this is the computer vision");
    let image = image::open("hook_lens\\input_img.png").unwrap();
    inference::infer(image);
}
