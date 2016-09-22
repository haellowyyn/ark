pub type Color = u32;


fn split(color: Color) -> (u32, u32, u32) {
    let red = color & 0xff;
    let green = (color >> 8) & 0xff;
    let blue = (color >> 16) & 0xff;
    (red, green, blue)
}

fn compose(red: u32, green: u32, blue: u32) -> Color {
    red | (green << 8) | (blue << 16)
}

pub fn blend(fgcolor: Color, bgcolor: Color, fgratio: u32) -> Color {
    if fgratio > 100 {
        // TODO return error
    }

    let bgratio = 100 - fgratio;
    let (fgr, fgg, fgb) = split(fgcolor);
    let (bgr, bgg, bgb) = split(bgcolor);

    let r = (fgr * fgratio + bgr * bgratio) / 100;
    let g = (fgg * fgratio + bgg * bgratio) / 100;
    let b = (fgb * fgratio + bgb * bgratio) / 100;

    compose(r, g, b)
}
