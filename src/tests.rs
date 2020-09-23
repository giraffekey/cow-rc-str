use crate::CowRcStr;

#[test]
pub fn test_creation() {
	let s: CowRcStr = String::from("Hello world").into();
	assert_eq!(s, "Hello world");
}
