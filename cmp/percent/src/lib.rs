//! Lib

#[cfg(test)]
mod test {

    #[test]
    fn fixed_encoder() {
        let poop = "💩💩💩💩💩💩";
        let mut e = codex_percent::FixedEncoder::<72>::init();
        let _f = e.encode(poop).unwrap();
        assert_eq!(
            core::str::from_utf8(e.cur_block()),
            Ok("%F0%9F%92%A9%F0%9F%92%A9%F0%9F%92%A9%F0%9F%92%A9%F0%9F%92%A9%F0%9F%92%A9")
        );
    }

    #[test]
    fn percent_encoder_encoder() {
        let poop = "💩💩💩💩💩💩";
        let s = percent_encoding::utf8_percent_encode(poop, percent_encoding::NON_ALPHANUMERIC)
            .to_string();
        assert_eq!(
            s,
            "%F0%9F%92%A9%F0%9F%92%A9%F0%9F%92%A9%F0%9F%92%A9%F0%9F%92%A9%F0%9F%92%A9"
        );
    }
}
