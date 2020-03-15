// primitive_types4.rs
// Get a slice out of Array a where the ??? is so that the test passes.
// Execute `rustlings hint primitive_types4` for hints!!

#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];
    // getting the arrays length
    let len : usize = a.len();
    // get range of an array
    let nice_slice = &a[1..4];

    // call this functions
    assert_eq!([2, 3, 4], nice_slice)
}
