//An attribute to hide warning for unused code
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

//A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

//A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

//Structs can be reused as fields of another struct
#[derive(Debug)]
struct Rectangle {
    //A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

// function witch calculates the area of a Rectangle
fn rect_area(rect: Rectangle) {
    let Rectangle {
        top_left: left,
        bottom_right: right,
    } = rect;

    let base = right.x - left.x;
    let height = left.y - right.y;
    println!("area is {}", base * height);
}

fn square(point: Point, arg: f32) -> Rectangle {
    Rectangle {
        top_left: Point { ..point },
        bottom_right: Point {
            x: point.x + arg,
            y: point.y - arg,
        },
    }
}

pub(crate) fn exec() {
    //Create struct with field init shorthand
    let name = String::from("Marcos");
    let age = 33;
    let marcos = Person { name, age };

    //Print debug struct
    println!("{:?}", marcos);

    // Instantiate a `Point`
    let point: Point = Point { x: 5.2, y: 0.4 };
    let another_point: Point = Point { x: 10.3, y: 0.2 };

    //Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    //Make a new point by using struct update syntax to use the fields of our other one
    let bottom_right = Point {
        x: 10.3,
        ..another_point
    };

    //`bottom_right.y` will be the same as `another_point.y` because we used that
    // from `another_point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    //Destructure the point using a `let` binding
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right,
    };

    //Instantiate a unit struct
    let _unit = Unit;

    //Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    //Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    //Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    println!("rectangle {:?}", _rectangle);
    rect_area(_rectangle);

    let _point = Point { x: 2.1, y: 5.1 };
    let factor: f32 = 5.0;
    let result_one = square(_point, factor);
    println!("Result is {:?}", result_one);
}
