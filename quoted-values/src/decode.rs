//! Decoding US-ASCII only quoted contextual values

use crate::DecodeError;

/// Utility to remove backslash characters.
pub struct BackslashRemoval;

impl BackslashRemoval {
    /// Returns quoting backslashes removed when these
    /// are used to quote '"' or '\' characters.
    ///
    /// @@Warning@@
    /// This is intended to operate in US-ASCII context only.
    /// @@Note@@
    /// Uses in-place removal due to no-alloc and
    /// whilst minimizing the memmoves so original
    /// input will be manipulated even if the result
    /// is an error.
    #[inline]
    pub fn in_place(mut input: &mut [u8]) -> Result<&mut [u8], DecodeError> {
        let len = input.len();

        match len {
            0 => return Ok(input),
            1 if input[0] == 92 => return Err(DecodeError::Unexpected(1)),
            1 if input[0] != 92 => return Ok(input),
            _ => {}
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

            let y_shift = match (input[x], input[x + 1]) {
                (92, 34) => true,
                (92, 92) => true,
                (92, _) => return Err(DecodeError::Unexpected(x + 1)),
                _ => false,
            };
            if y_shift {
                if skip_mark != -1 {
                    let skip_start = skip_mark as usize + 1;
                    let skip_end = x;

                    input.copy_within(skip_start..skip_end, skip_mark as usize);
                }

                skip_mark = (x - skips) as isize;
                skips += 1;
                t_next = true;
            }
        }

        if skip_mark != -1 {
            let skip_start = skip_mark as usize + skips;
            let skip_end = input.len();

            input.copy_within(skip_start..skip_end, skip_mark as usize);
        }
        let ret = match input.split_off_mut(..input.len() - skips) {
            Some(r) => r,
            None => return Err(DecodeError::Bug("split_off_mut ret None?!")),
        };

        if ret[ret.len() - 1] == 92 {
            return Err(DecodeError::Unexpected(ret.len()));
        }

        Ok(ret)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("", "")]
    #[case("FOO\\\"BAR", "FOO\"BAR")]
    #[case("FOO\\\"BAR\\\"BAZ", r##"FOO"BAR"BAZ"##)]
    fn okays(#[case] input: String, #[case] expected: String) {
        let mut bytes = input.into_bytes();
        let skipped = BackslashRemoval::in_place(&mut bytes).unwrap();
        assert_eq!(expected.as_str(), core::str::from_utf8(skipped).unwrap());
    }

    #[rstest]
    #[case("\\", DecodeError::Unexpected(1))]
    #[case("\\FOOBAR", DecodeError::Unexpected(1))]
    #[case("FOO\\BAR", DecodeError::Unexpected(4))]
    #[case("FOO\\\\\\BAR", DecodeError::Unexpected(6))]
    #[case("FOOBAR\\", DecodeError::Unexpected(7))]
    fn nasties(#[case] input: String, #[case] err: DecodeError) {
        let mut bytes = input.into_bytes();
        let res = BackslashRemoval::in_place(&mut bytes);
        assert_eq!(Err(err), res);
    }
}
