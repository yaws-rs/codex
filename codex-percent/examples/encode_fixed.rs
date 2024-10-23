use codex_percent::FixedEncoder;

fn main() {
    let s = "💩";

    let mut e = FixedEncoder::<12>::init();
    let res = e.encode(s).unwrap();

    assert_eq!(res, 12);
    let t = core::str::from_utf8(e.cur_block().as_slice());

    assert_eq!(t, Ok("%F0%9F%92%A9"));

    println!("{} -> {}", s, t.unwrap());
}
