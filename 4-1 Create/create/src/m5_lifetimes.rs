fn example_0() {
    let r:&i32;
    
    let x:i32 = 5;
    r = &x;
    
    println!("r: {}", r);
}

fn example_1() {

    //Allocate space in memory
    let highest_age:i32;

    //Initialize variables
    let alice_age:i32 = 20;
    let bob_age:i32 = 22;

    //Call function
    highest_age = largest(&alice_age, &bob_age);

    //print output
    println!("Highest age is {}", highest_age);

    fn largest(compare_1:&i32, compare_2:&i32) -> i32 {
        if compare_1 > compare_2 {
            return *compare_1
        } else {
            return *compare_2
        }
    }
}

fn example_2() {

    //Allocate space in memory
    let highest_age:&i32;

    //Initialize variables
    let alice_age:i32 = 20;
    let bob_age:i32 = 22;

    //Call function
    highest_age = largest(&alice_age, &bob_age);

    //print output
    println!("Highest age is {}", highest_age);

    fn largest<'a>(compare_1:&'a i32, compare_2:&'a i32) -> &'a i32 {
        if compare_1 > compare_2 {
            return compare_1
        } else {
            return compare_2
        }
    }
}

//This example to show how to get around lifetimes
fn example_3() {

    //Allocate space in memory
    let highest_age:&i32;
    let new_value:i32;

    //Initialize variables
    let alice_age:i32 = 20; //'a

    {
        let bob_age:i32 = 22; //'b

        //Call function
        highest_age = largest(&alice_age, &bob_age);
        new_value = *highest_age; //this is the solution because new_value has lifetime 'a

    } //'b goes out of scope here

    //print output
    println!("Highest age is {}", new_value);

    fn largest<'a>(compare_1:&'a i32, compare_2:&'a i32) -> &'a i32 {
        if compare_1 > compare_2 {
            return compare_1
        } else {
            return compare_2
        }
    }
}

fn example_4_generics() {

    //Allocate space in memory
    let highest_age:&i32;

    //Initialize variables
    let alice_age:i32 = 20;
    let bob_age:i32 = 22;

    //Call function
    highest_age = largest(&alice_age, &bob_age);

    //print output
    println!("Highest age is {}", highest_age);

    fn largest<'a, T:PartialOrd>(compare_1:&'a T, compare_2:&'a T) -> &'a T {
        if compare_1 > compare_2 {
            return compare_1
        } else {
            return compare_2
        }
    }
}

struct Person<'p> {
    name:&'p str,
    points: &'p f32
}

fn example_5_struct() {

    //Allocate space in memory
    let highest_age:&f32;

    //Initialize variables
    let alice:Person = Person {name: "Alice", points: &50.2};
    let bob:Person = Person {name: "Bob", points: &40.5};

    //Call function
    highest_age = largest::<f32>(alice.points, bob.points);

    //print output
    println!("Highest age is {}", highest_age);

    fn largest<'a, T:PartialOrd>(compare_1:&'a T, compare_2:&'a T) -> &'a T {
        if compare_1 > compare_2 {
            return compare_1
        } else {
            return compare_2
        }
    }
}