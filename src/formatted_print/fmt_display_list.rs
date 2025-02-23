use std::fmt;
//https://doc.rust-lang.org/rust-by-example/hello/print/print_display/testcase_list.html
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;
        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{count}: {}", v)?;
        }

        write!(f, "]")
    }
}

pub(crate) fn exec() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}