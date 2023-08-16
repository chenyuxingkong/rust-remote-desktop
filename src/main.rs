use scrap::{Display, Capturer, Frame};


fn main() {
    let display = Display::primary().expect("TODO: panic message");
    display.height();
    display.width();
    let capturer = Capturer::new(display).expect("TODO: panic message");
    capturer.width();
    capturer.height();
    print!("Test");
}
