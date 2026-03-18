use quoted_values::BackslashRemoval;

fn main() {
    let mut bytes = "FOOBAR\\\"".to_string().into_bytes();
    let skipped = BackslashRemoval::in_place(&mut bytes).unwrap();

    println!("Removed = {}", core::str::from_utf8(skipped).unwrap());
}
