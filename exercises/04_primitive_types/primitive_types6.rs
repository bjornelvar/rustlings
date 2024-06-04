// primitive_types6.rs
//
// Use a tuple index to access the second element of `numbers`. You can put the
// expression for the second element where ??? is so that the test passes.
//
// Execute `rustlings hint primitive_types6` or use the `hint` watch subcommand
// for a hint.

#[test]
fn indexing_tuple() {
    let numbers = (1, 2, 3);

    // Lausn 1
    let (first, second, third) = numbers;

    // Lausn 2 (Réttari lausn)
    let second = numbers.1;

    assert_eq!(2, second,
        "This is not the 2nd number in the tuple!")
}
