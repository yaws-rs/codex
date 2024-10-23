use codex_percent::VecEncoder;

fn main() {
    let s = "💩";

    let mut v: Vec<u8> = Vec::with_capacity(12);
    let res = VecEncoder::encode(s, &mut v).unwrap();

    assert_eq!(res, 12);
    let t = core::str::from_utf8(v.as_slice());

    assert_eq!(t, Ok("%F0%9F%92%A9"));

    println!("{} -> {}", s, t.unwrap());
}
