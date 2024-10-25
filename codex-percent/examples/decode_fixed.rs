use codex_percent::FixedDecoder;

fn main() {
    let s = "%F0%9F%92%A9";

    let mut d = FixedDecoder::<4>::init();
    let res = d.decode(s).unwrap();

    assert_eq!(res, 4);
    let t = core::str::from_utf8(d.cur_block().as_slice());

    assert_eq!(t, Ok("💩"));

    println!("{} -> {}", s, t.unwrap());
}
