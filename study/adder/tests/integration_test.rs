use adder::ad::add_two;

#[test]
fn it_adds_two() {
    let result = add_two(4);
    assert_eq!(result, 6);
}
