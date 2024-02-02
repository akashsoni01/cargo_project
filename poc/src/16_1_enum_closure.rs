
pub enum YourEnum {
    CASEONE,
    CASETOW,
  }
  
impl YourEnum {
    // THE BELOW CODE IS USED TO CONVERT THE ENUM TO A CLOSURE 
    // EXAMPLE: 
    // let closure = YourEnum::CASEONE.as_closure();
    // let result = closure((1, 2));
    // assert_eq!(result, true);
  pub fn as_closure<T>(&self) -> impl Fn((T, T)) -> bool
  where
    T: Ord,
  {
    match self {
        YourEnum::CASEONE => |(a, b)| a <= b,
        YourEnum::CASETOW => |(a, b)| a >= b,
    }
  }
}


// usecase with generic data type 
pub fn is_sorted<I>(items: I, direction: YourEnum) -> bool
where
  I: IntoIterator,
  I::Item: Ord + Clone,
{
  items
    .into_iter()
    .tuple_windows()
    .all(direction.as_closure())
}

#[test]
fn test_is_sort_assertion() {
  let a = vec![1, 20, 3];
  let b = vec![1, 2, 60];
  let c = vec![100, 20, 6];
  let d = vec![100, 20, 60];
  assert!(!is_sorted(a, YourEnum::CASEONE));
  assert!(is_sorted(b, YourEnum::CASEONE));
  assert!(is_sorted(c, YourEnum::CASETOW));
  assert!(!is_sorted(d, YourEnum::CASETOW))
}


// other application 
/*
  assert!(is_sorted(b, Direction::ASC));
  assert!(is_sorted(c, Direction::DESC));
*/
