// Integration testing is for testing your library as it would be used externally

use testing;

#[test]
fn it_adds_two() {
	assert_eq!(4, testing::add_two(2));
}
