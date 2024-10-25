use codex_percent::VecDecoder;

fn main() {
    let s = "%F0%9F%92%A9";

    let mut v: Vec<u8> = Vec::with_capacity(4);
    let res = VecDecoder::decode(s, &mut v).unwrap();

    assert_eq!(res, 4);
    let t = core::str::from_utf8(v.as_slice());

    assert_eq!(t, Ok("💩"));

    println!("{} -> {}", s, t.unwrap());
}
