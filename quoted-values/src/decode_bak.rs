//! Decoding quoted-values

use crate::DecodeError;

// \\ = 92, 92 => 92
// \" = 92, 34 => 34

pub struct BackslashRemoval {
}

impl BackslashRemoval {

    #[inline]
    pub fn in_place(mut input_orig: &mut [u8]) -> Result<&mut [u8], DecodeError> {

        let mut input = &mut input_orig;
        let len = input.len();

        if len < 2 {
            return Ok(input);
        }
        
        let end = len - 1;

        let mut skip_mark: isize = -1;

        let mut t_next = false;

        let mut skips: usize = 0;
        
        for x in 0..end {
            if t_next {
                t_next = false;
                continue;
            }
            let y_shift = match (input[x], input[x+1]) {
                (92, 34) => true,
                (92, 92) => true,
                (92, _) => panic!("wtf"),
                _ => false,
            };
            if y_shift {
                input[x] = 0;

                if skip_mark != -1 {

                    let skip_start = skip_mark as usize + 1;
                    let skip_end = x;

                    input.copy_within(skip_start..skip_end, skip_mark as usize);
                    skip_mark = (x as isize) - (skips) as isize;
                }
                else {
                    skip_mark = x as isize;
                }
                skips+=1;
                t_next = true;
            }
        }

        if skip_mark != -1 {

            let skip_start = skip_mark as usize + skips;
            let skip_end = input.len();

            input.copy_within(skip_start..skip_end, skip_mark as usize);
        }
        let ret = input.split_off_mut(..input.len()-skips).unwrap();

        Ok(input)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use rstest::rstest;
    
    #[rstest]
    #[case("", "", 0)]
    #[case("FOO\\\"BAR", "FOO\"BAR", 1)]
    #[case("FOO\\\"BAR\\\"BAZ", r##"FOO"BAR"BAZ"##, 2)]
    #[case("FOO\\\\\\BAR", r##"FOO\\\BAR"##, 1)]
    fn simples(#[case] mut input: String, #[case] expected: String, #[case] skips: usize) {
        let mut bytes = input.into_bytes();
        let skipped = BackslashRemoval::in_place(&mut bytes).unwrap();
        assert_eq!(expected.as_str(), core::str::from_utf8(skipped).unwrap());
    }
}
