#[derive(Debug)]
pub struct Frame<'a> {
    pub kind: u8,
    pub payload: &'a [u8],
}

pub fn parse(buf: &[u8]) -> Option<(Frame<'_>, &[u8])> {
    let (&kind, rest) = buf.split_first()?;
    let len_bytes: [u8; 2] = rest.get(..2)?.try_into().ok()?;
    let len = usize::from(u16::from_be_bytes(len_bytes));
    let payload = rest.get(2..2 + len)?;
    Some((Frame { kind, payload }, &rest[2 + len..]))
}

fn main() {
    let buf: Vec<u8> = vec![1, 0, 3, b'a', b'b', b'c', 2, 0, 1, b'z'];
    let mut rest = buf.as_slice();
    while let Some((frame, tail)) = parse(rest) {
        println!("{} {:?}", frame.kind, frame.payload);
        rest = tail;
    }
}
