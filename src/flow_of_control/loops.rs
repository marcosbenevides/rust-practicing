pub(crate) fn exec() {
    let mut count = 0u32;

    println!("let's count until infinity!");

    // Infinite loop
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            //Skip the rest of this iteration
            continue;
        }

        println!("{}", count);
        if count == 5 {
            println!("Ok, that's enough");

            //Exit this loop
            break;
        }
    }

    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // This would break only the inner loop
            // break;

            // This breaks the outer loop
            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);

    //for and iterators

    {
        let names = vec!["Bob", "Frank", "Ferris"];

        //preserve the values from collection to use after
        for name in names.iter() {
            match name {
                &"Ferris" => println!("There is a rustacean among us!"),
                // TODO ^ Try deleting the & and matching just "Ferris"
                _ => println!("Hello {}", name)
            };
        }
        println!("names: {:?}", names);
    };

    {
        let names = vec!["Bob", "Frank", "Ferris"];

        //consumes the collection
        for name in names.into_iter() {
            match name {
                "Ferris" => println!("There is a rustacean among us!"),
                _ => println!("Hello {}", name)
            }
        };
        // Compile error
        //println!("names: {:?}", names);
    };

    {
        let mut names = vec!["Bob", "Frank", "Ferris"];

        // can modify the collection
        for name in names.iter_mut() {
            *name = match name {
                &mut "Ferris" => "There is a rustacean among us!",
                _ => "Hello"
            };
        }
        println!("names: {:?}", names);
    }
}
