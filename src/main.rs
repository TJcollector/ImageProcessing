use std::time::{SystemTime, UNIX_EPOCH};

mod Filters {
    pub mod sharpness;
    pub mod blurring;
    pub mod darken;
    pub mod brightness;
}

use Filters::sharpness::Sharpenimage;
use Filters::blurring::blur;
use Filters::darken::darken;
use Filters::brightness::brightness;
fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    //Copy and paste your picture path here!!
    let s="src/images (2).jpg"; //example
    Sharpenimage(s);
    blur(s);
    darken(s);
    brightness(s);
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("{:?}", end.as_secs());
}
