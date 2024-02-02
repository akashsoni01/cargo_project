
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
