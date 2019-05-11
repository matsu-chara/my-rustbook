use std::io::Cursor;
use wordcount::{count, CountOption};

#[macro_use]
mod utils;

#[test]
fn char_count_works() {
    let input = Cursor::new(b"abadracadabra");

    let freq = count(input, CountOption::Char);
    assert_map!(freq,
    {
      "a" => 6,
      "b" => 2,
      "c" => 1,
      "d" => 2,
      "r" => 2
    }
    );
}

#[test]
fn char_count_utf8() {
    let input = Cursor::new(
        r#"
あいうえお
かきくけこ
"#
    );

    let freq = count(input, CountOption::Char);

    assert_eq!(freq.len(), 10);
    for (_, count) in freq {
        assert_eq!(count, 1);
    }
}
