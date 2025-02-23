#![allow(dead_code)]

use crate::use_declaration::Role::*;
use crate::use_declaration::Stage::*;

enum Stage {
    Beginner,
    Advanced,
}

enum Role {
    Student,
    Teacher,
}

pub(crate) fn exec() {
    //Explicitly `use` each name so they are available without
    // manual scoping
    //use crate::Stage::{Advanced, Beginner};
    // Automatically `use` each name inside `Role`
    //use crate::Role::*;

    //Equivalent to `Stage::Beginner`
    let stage = Beginner;
    // Equivalent to `Role::Student`
    let role = Student;

    match stage {
        Beginner => println!("Beginners are starting their learning journey!"),
        Advanced => println!("Advanced learners are mastering their subjects...!"),
    }

    match role {
        Student => println!("Students are acquiring knowledge"),
        Teacher => println!("Teacher are spreding knowledge"),
    }
}
