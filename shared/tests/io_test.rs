use shared::{read_lines, read_symbol_separated_items};

#[test]
fn read_lines_test() {
    let lines = read_lines("tests/fixtures/1.dat").unwrap();

    assert_eq!(lines.len(), 5)
}

#[test]
fn read_symbol_separated_data_test() {
    let items = read_symbol_separated_items("tests/fixtures/2.dat", ',').unwrap();

    assert_eq!(items.len(), 4);
}
