use scrap::{Display, Capturer};


fn main() {
    let display = Display::primary().expect("TODO: panic message");
    display.height();
    display.width();
    let capturer = Capturer::new(display).expect("TODO: panic message");
    capturer.width();
    capturer.height();
    println!("scrap How to use?");
    print!("Test");
}
