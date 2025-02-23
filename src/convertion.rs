use std::convert::{From, Into, TryFrom, TryInto};
use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

// the `Into` implementation it`s not needed when implementing `From`
// but the opposite is not true
/*impl Into<Number> for i32 {
    fn into(self) -> Number {
        Number { value: self }
    }
}*/

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

// `TryFrom` and `TryInto` its like the previous examples, but can be
// `Ok` or `Error`
impl TryFrom<i32> for EvenNumber {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

// Converting `String`
#[derive(Debug)]
struct Circle {
    radius: i32,
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

impl FromStr for Circle {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().parse() {
            Ok(num) => Ok(Circle { radius: num }),
            Err(e) => Err(e),
        }
    }
}

pub(crate) fn exec() {
    //Simple conversion
    let my_str = "Hello world!";
    let mu_string = String::from(my_str);
    println!("{}", mu_string);

    //use custom `From` implementation
    let num = Number::from(30);
    println!("My number is {:?}", num);

    //use custom `Into` implementation
    let int = 5;
    let number: Number = int.into();
    println!("My number is {:?}", number);

    //TryFrom
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    //TryInto
    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));

    // Converting `String`
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());

    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();
    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);

    let radius = "    3";
    let circle: Circle = radius.parse().unwrap();
    println!("{:?}", circle);

    // Will get error
    
    // let radius_two = "L";
    // let circle_two: Circle = radius_two.parse().unwrap();
    // println!("{:?}", circle_two);
}
