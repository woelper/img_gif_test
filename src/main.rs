use image;
use image::codecs::gif::GifDecoder;
use image::AnimationDecoder;
use reqwest;

const URL: &str = "https://user-images.githubusercontent.com/3613791/198876501-61f76e9c-ef0f-46fa-8e4b-a5e5b20fcecf.gif";

fn main() {
    let img_bytes = reqwest::blocking::get(URL).unwrap().bytes().unwrap();
    let cursor = std::io::Cursor::new(img_bytes);
    let gif_decoder = GifDecoder::new(cursor).unwrap();

    // will fail because of "unknown block type encountered"
    // let frames = gif_decoder.into_frames().collect_frames().unwrap();

    for f in gif_decoder.into_frames() {
        match f {
            Ok(_) => println!("Frame decoded OK"),
            Err(e) => println!("{e}"),
        }
        // program will hang here forever
    }
}
