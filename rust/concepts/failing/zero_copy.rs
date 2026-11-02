pub struct Frame<'a> {
    pub kind: u8,
    pub payload: &'a [u8],
}

fn read_frame() -> Frame<'static> {
    let buf = vec![1, 0, 1, b'x'];
    Frame { kind: buf[0], payload: &buf[3..] } // buf is freed when this function returns
}

fn main() {
    println!("{}", read_frame().kind);
}
