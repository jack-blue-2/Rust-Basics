// This lets Rust know the crate exists, and tells it the structure of the files.
// utils doesn't use the pub keyword because it's not needed for modules.
mod utils {
    pub mod random_crate;
}

// This imports the function from the crate.
// If you didn't have this line here, you'd have to call it with
// utils::random_crate::random_integer() every time, but now you can just
// call random_integer() directly.
use utils::random_crate::random_integer;

// To use HashMap, you must import it from the standard library.
use std::collections::HashMap;


fn add(a: i32, b: i32) -> i32 {
  return a + b;
}

// In Rust, you can omit the return keyword. Just write the value on the last
// line of the function, without a semicolon. It's up to you which style you prefer.

fn add1(a: i32, b: i32) -> i32 {
  a + b
}

// If you add a semicolon at the end of the last line, it becomes a statement
// that returns (), which is the unit type (similar to None in Python).

// The top file is always called main.rs, and the top function is always called main.
// The main function is always at the bottom of the file.

fn main() {

    let sum = add(5, 10);
    println!("The sum is: {}", sum);

    let rand_num = random_integer(100);
    println!("The random number is: {}", rand_num);

    let x = 5;
    println!("This is a static text.");
    println!("The value of x is: {}", x);
    println!("{}", x);

    let name = "John"; // By default, variables are immutable
    let mut surname = "Doe";
    // name = "Alice"; // This line would cause a compile-time error
    surname = "Smith"; // This is allowed because surname is mutable
    let age = 30;
    println!("{} {} is {} years old.", name, surname, age);

    // The type of a variable is decided by the value you give it.
    // Rust looks at the value and automatically chooses the right type:
    let my_num = 5;         // integer
    let my_double = 5.99;   // float
    let my_letter = 'D';    // character. Single quotes for single characters
    let my_bool = true;     // boolean
    let my_text = "Hello";  // string. Double quotes for string literals

    // You can also specify the type explicitly:
    let my_explicit_num: i32 = 5; // integer
    let my_explicit_double: f64 = 5.99; // float
    let my_explicit_letter: char = 'D'; // character
    let my_explicit_bool: bool = true; // boolean
    let my_explicit_text: &str = "Hello"; // string

    // Constant variables are used to store values that never change.
    // They must be defined with a type, and are usually written in uppercase letters:
    const MY_CONSTANT: i32 = 100;

    // A string literal is immutable:
    let greeting: &str = "Hello, World!";

    // A String is mutable. There are many ways to create a String:
    let mut mutable_greeting: String = String::from("Hello");
    let another_greeting: String = "Hello".to_string();
    let mut yet_another_greeting: String = String::new(); // empty String
    
    // push_str is a method that adds text to the end of a String.
    mutable_greeting.push_str(", World!");

    // push is a method that adds a single character to the end of a String.
    mutable_greeting.push('!');

    // you can combine Strings using the + operator or the format! macro:
    let combined = mutable_greeting + " How are you?"; // Note: mutable_greeting is moved here and can no longer be used
    let name = "Alice";
    let formatted = format!("Hello, {}! Welcome to Rust.", name); // Does not take ownership
    println!("{}", combined);
    println!("{}", formatted);

    // The + operator takes ownership of the left operand.
    // If you need to keep using the original String, use format! instead.
    let another_formatted = format!("Hello, {}! Welcome to Rust.", name);
    println!("{}", another_formatted);

    // You can only add a &str to a String with +.

    // The format! macro is very flexible and can take multiple arguments.
    let x = 5;
    let y = 10;
    let formatted = format!("The sum of {} and {} is {}.", x, y, x + y);
    println!("{}", formatted);

    // You can use the .len() method to get the length of a string:
    let length = another_greeting.len();
    println!("The length of the greeting is: {}", length);

    // You can use the .is_empty() method to check if a String is empty:
    let is_empty = yet_another_greeting.is_empty();
    println!("Is the yet_another_greeting empty? {}", is_empty);

    // You cannot directly index into a String in Rust.
    // In other languages, there's an index at each byte. However, in Rust,
    // Strings are UTF-8 encoded, which means that sometimes an
    // index takes up more than one byte.
    // Therefore, Rust does not allow direct indexing to avoid confusion and errors.

    // You can use indexing to access individual characters in a String
    // using the .chars() method to get an iterator over the characters,
    // then the .nth() method to get the character at a specific index:
    let first_char = another_greeting.chars().nth(0).unwrap(); // Get the first character
    println!("The first character is: {}", first_char);
    
    // You can replace parts of a String using the .replace() method:
    let new_greeting = another_greeting.replace("World", "Rust");
    println!("New greeting: {}", new_greeting);

    // you can replace a letter at a particular index by converting the String to a vector of characters,
    // modifying the character at that index, and then converting it back to a String:
    let mut chars: Vec<char> = another_greeting.chars().collect();
    chars[3] = 'R'; // Change 'W' to 'R'
    let modified_greeting: String = chars.into_iter().collect();
    println!("Modified greeting: {}", modified_greeting);

    // Arrays
    // The size of an array is fixed. You cannot add or remove elements.
    // All the values in an array must be of the same type.
    let mut fruits: [&str; 3] = ["Apple", "Banana", "Cherry"];
    println!("The first fruit is: {}", fruits[0]);
    fruits[1] = "Blueberry"; // Change "Banana" to "Blueberry"
    println!("The second fruit is now: {}", fruits[1]);

    // When printing the whole array, you must use {:?} inside println!:
    println!("The fruits array is: {:?}", fruits);
    
    // Vectors
    // Vectors are similar to arrays, but they can grow and shrink in size.
    // Vectors must also contain values of the same type.
    let mut veggies: Vec<&str> = vec!["Carrot", "Potato", "Cucumber"];

    // Vec<T> is the syntax for a vector that holds values of type T.
    // In this case, Vec<&str> is a vector of string slices.

    println!("The first vegetable is: {}", veggies[0]);
    veggies.push("Tomato"); // Add a new element
    println!("The last vegetable is now: {}", veggies[veggies.len() - 1]);
    // You can also remove the last element using pop():
    veggies.pop();
    println!("After popping, the last vegetable is: {}", veggies[veggies.len() - 1]);
    // pop() returns an Option type, which is like Optional in Python.
    // If the vector is empty, pop() returns None. Otherwise, it returns Some(value).
    // Use insert() to add an item at a specified index:
    veggies.insert(0, "Lettuce"); // Add "Lettuce" at the beginning
    println!("After inserting, the first vegetable is: {}", veggies[0]);
    // You can remove an entry at a specific index using remove():
    veggies.remove(0); // Remove the first element
    println!("After removing the first vegetable, the new first vegetable is: {}", veggies[0]);
    // You can change the size of a vector using the resize() method:
    veggies.resize(5, "Lettuce"); // Resize to 5 elements, filling new slots with "Lettuce"
    println!("After resizing, the vector has {} elements.", veggies.len());
    // You can change the value at a specific index:
    veggies[1] = "Spinach"; // Change the second element
    println!("The second vegetable is now: {}", veggies[1]);
    // You can also clear all elements from a vector using clear():
    veggies.clear();
    println!("After clearing, the vector has {} elements.", veggies.len());

    // Just like arrays, you can use a for loop to go through all the values in a vector:
    for veggie in veggies.iter() {
        println!("{}", veggie);
    }
    // Here's another way of doing the same thing:
    for veggie in &veggies {
        println!("I like {}.", veggie);
    }
    // Note: Use &fruits to borrow the vector instead of moving it.
    // In Rust, borrowing means using a reference to a value instead of taking ownership of it.
    // When you loop through a vector without &, the values are moved out, and you can no longer
    // use the vector. But when you borrow the vector using &, you can still use it later in your program.

    // Vectors and arrays both allow indexing to access elements.
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("The first number is: {}", numbers[0]);
    numbers[2] = 10; // Change the third element from 3 to 10
    println!("The third number is now: {}", numbers[2]);

    // Tuples
    // Tuples can hold values of different types.
    // The size of a tuple is fixed. You cannot add or remove elements.
    // You cannot replace an element in a tuple.
    let person: (&str, i32, bool) = ("Alice", 30, true);
    println!("Name: {}", person.0);
    println!("Age: {}", person.1);
    println!("Is student: {}", person.2);
    // You can also destructure/unpack a tuple into separate variables:
    let (name, age, is_student) = person;
    println!("Destructured - Name: {}, Age: {}, Is student: {}", name, age, is_student);
    // You can change the values in a mutable tuple:
    let mut mutable_person: (&str, i32, bool) = ("Bob", 25, false);
    mutable_person.1 = 26; // Change age from 25 to 26
    println!("Updated age: {}", mutable_person.1);

    // Tuples are often used to return multiple values from a function:
    fn get_person() -> (&'static str, i32, bool) {
        ("Charlie", 35, true)
    }
    let (name, age, is_student) = get_person();
    println!("Returned - Name: {}, Age: {}, Is student: {}", name, age, is_student);

    // HashMaps
    // HashMaps store key-value pairs.
    // The keys and values can be of different types.
    // HashMaps can grow and shrink in size.
    // To use HashMap, you must import it from the standard library.
    let mut scores: HashMap<&str, i32> = HashMap::new();
    scores.insert("Alice", 90);
    scores.insert("Bob", 85);
    println!("Alice's score: {}", scores.get("Alice").unwrap());
    // unwrap means "give me the value inside the Option". If the key
    // doesn't exist, it will cause a panic. In real code, you should handle this more gracefully.
    // There's an explanation of error handling later in this file.
    // You can also use the get method without unwrap:
    // println!("Alice's score: {}", scores.get("Alice"));
    // This would error because it returns an Option type.
    // In order to print it, you need to use {:?}
    println!("Alice's score: {:?}", scores.get("Alice"));
    scores.insert("Alice", 95); // Update Alice's score
    println!("Alice's updated score: {}", scores.get("Alice").unwrap());
    scores.remove("Bob"); // Remove Bob's score
    match scores.get("Bob") {
        Some(score) => println!("Bob's score: {}", score),
        None => println!("Bob's score not found."),
    }
    // You can use indexing syntax to access values by key:
    println!("Alice's score (via indexing): {}", scores["Alice"]);
    // If you insert a new value using a key that already exists, the old value is replaced with the new one:
    scores.insert("Alice", 100);
    println!("Alice's score after reinserting: {}", scores["Alice"]);
    // This is how you print out the whole HashMap, with {:?}:
    println!("Scores HashMap: {:?}", scores);

    // You can also use a for loop to loop through all the key/value pairs in a hashmap:
    let mut capitalCities = HashMap::new();

    // Add keys and values (Country, City)
    capitalCities.insert("England", "London");
    capitalCities.insert("Germany", "Berlin");
    capitalCities.insert("Norway", "Oslo");

    // Loop through the HashMap
    for (country, city) in &capitalCities {
    println!("The capital of {} is {}.", country, city);
    }

    // Operators
    let sum = 5 + 10;           // Addition
    let difference = 95.5 - 4.3; // Subtraction
    let product = 4 * 30;       // Multiplication
    let quotient = 56.7 / 32.2; // Division
    let remainder = 43 % 5;     // Modulus (remainder)

    // These are the same as Python: +=, -=, *=, /=, %=
    // So are these: ==, !=, >, <, >=, <=

    let logged_in = true;
    let is_admin = false;

    println!("Is regular user: {}", logged_in && !is_admin);
    println!("Has any access: {}", logged_in || is_admin); // True if at least one is true
    println!("Not logged in: {}", !logged_in);

    // Control Flow
    let score = 85;

    if score >= 90 {
        println!("Grade: A");
    } else if score >= 80 {
        println!("Grade: B");
    } else if score >= 70 {
        println!("Grade: C");
    } else {
        println!("Grade: F");
    }

    // You can assign the result of an if to a variable
    // BUT they must return the same type!
    let time = 20;
    let greeting = if time < 18 {
        "Good day."
    } else {
        "Good evening."
    };

    // You can also write it in one line.
    // Again, they must return the same type!
    let greeting_one_liner = if time < 18 { "Good day." } else { "Good evening." };
    
    // match is like a switch statement in other languages:
      let day = 4;

    match day {
        1 => println!("Monday"),
        2 => println!("Tuesday"),
        3 => println!("Wednesday"),
        4 => println!("Thursday"),
        5 => println!("Friday"),
        6 => println!("Saturday"),
        7 => println!("Sunday"),
        _ => println!("Invalid day."),
    }

    // You can also match on multiple patterns:
    let x = 1;

    match day {
        1 | 2 | 3 | 4 | 5 => println!("Weekday"),
        6 | 7 => println!("Weekend"),
        _ => println!("Invalid day"),
    }

    // Match can also return a value:
    let number = 4;

    let description = match number {
        1 => "One",
        2 => "Two",
        3 => "Three",
        4 => "Four",
        _ => "Unknown",
    };

    // Rust has three types of loops: loop, while, and for.

    // Loop runs forever until you explicitly stop it.
    let mut count = 0;
    loop {
        println!("Hello World!");

        if count == 3 {
            break;
        }

        count += 1;
    }

    // You can also return a value from a loop using break with a value.
    let result = loop {
        count += 1;

        if count == 5 {
            break count * 2; // Return double the count
        }
    };

    // The while loop runs as long as a condition is true.
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    // Using break and continue with while loops
    let mut count = 0;
    while count < 5 {
        count += 1;

        if count == 3 {
            continue; // Skip the rest of the loop when count is 3
        }

        if count == 4 {
            break; // Exit the loop when count is 4
        }

        println!("Count: {}", count);
    }

    // You cannot return a value from a while or for loop using break like you can with a loop.

    // The for loop is used to iterate over a range or collection.
    for i in 1..6 { // 1 to 5
        println!("{}", i);
    }

    for i in 1..=5 { // 1 to 5 inclusive
        println!("{}", i);
    }

    // Rust handles the counter variable (i) automatically,
    // unlike many other programming languages. You don't need to declare
    // or increment it manually.

    // You can also loop through arrays and vectors using for:
    let animals = ["Dog", "Cat", "Rabbit"];
    for animal in animals.iter() {
        println!("{}", animal);
    }

    // continue and break also work in for loops.

    // Rust also has an enumerate function:
    let fruits = ["Apple", "Banana", "Cherry"];
    for (index, fruit) in fruits.iter().enumerate() {
        println!("{}: {}", index, fruit);
    }

    // the iter() method returns an iterator over the array,
    // and enumerate() adds a counter to it, giving us both the index and the value.

    // Each value has one owner, usually a variable.
    // When the owner goes out of scope, the value is deleted.
    // This is called ownership, and it helps manage memory safely without a
    // garbage collector.
    // You can only have one owner for a value at a time, unless you borrow it.
    // In this example, a owns the string. Then we move it to b:
    let a = String::from("Hello");
    let b = a;
    // println!("{}", a); Error: a no longer owns the value
    println!("{}", b); // Ok: b now owns the value

    // However, simple types like numbers, characters and booleans are copied, not moved.
    // This means you can still use the original variable after assigning it to another:
    let mut x = 5;
    let y = x;
    println!("x: {}, y: {}", x, y);
    x += 1;
    println!("After changing x: x: {}, y: {}", x, y); // x has changed, y is unchanged.

    // For other types, like String, if you really want to keep the original value and
    // also assign it to another variable, you can use the .clone() method, which makes a copy of the data:
    let a = String::from("Hello");
    let b = a.clone();
    println!("a: {}, b: {}", a, b);

    // However, if you don't need to own the value twice, using a reference (&) is usually better
    // than cloning. A reference lets you borrow a value without owning it.
    let a = String::from("Hello");
    let b = &a; // b is a reference to a
    println!("a: {}, b: {}", a, b); // Both can be used

    // Why would you want to borrow a value instead of owning it?
    // Borrowing is useful when you want to read or use a value without taking ownership of it.
    // This way, the original owner can still use the value, and you avoid unnecessary
    // copying of data, which can be inefficient for large values.

    // What stops race conditions when using references?
    // Rust enforces rules at compile time to ensure memory safety:
    // 1. You can have either one mutable reference or any number of immutable
    // references to a value at a time.
    // 2. References must always be valid.

    // If you want to change a value through a reference, you need to make the reference mut:
    let mut a = String::from("Hello");
    let b = &mut a; // b is a mutable reference to a

    // This looks like two mutable references to a, but it's not.
    // This is because a is not being used while b exists.

    b.push_str(", World!"); // Change the value through the reference

    // This would change the value of a as well as b. However,
    // Rust doesn't allow you to use a while there's a mutable reference to it.
    // So this next line would cause an error:

    // println!("a: {}", a);

    // But this would be fine:
    println!("b is now a reference to a: {}", b);
    // To stop using b and use a again, you can create a new scope:
    {
        let b = &mut a; // mutable borrow in a new scope
        b.push_str(" How are you?"); // use mutable reference
        println!("b inside scope: {}", b); // use mutable reference
    } // mutable borrow ends here
    println!("a: {}", a); // now you can use 'a' again

    // Error handling in Rust is done using the Result and Option types.
    // Result is used for operations that can succeed or fail.
    // Option is used for values that can be present or absent. It's like Optional in Python.
    // Both types are enums that help handle errors and missing values safely.
    // They encourage you to think about error handling and avoid panics.

    // Here's an example of using Result:
    fn divide(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err(String::from("Cannot divide by zero")) // This returns an error
        } else {
            Ok(a / b) // Ok is part of the standard library
        }
    }
    // The line with OK doesn't end with a semicolon because it's returning a value.
    // If you add a semicolon, it becomes a statement that returns (),
    // which is the equivalent of None in Python.

    match divide(10.0, 2.0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    // Here's an example of using Option:
    fn get_nth_element(vec: &Vec<i32>, n: usize) -> Option<i32> {
        if n < vec.len() {
            Some(vec[n])
        } else {
            None
        }
    }

    let numbers = vec![10, 20, 30];
    match get_nth_element(&numbers, 1) {
        Some(value) => println!("Value: {}", value),
        None => println!("No value found at that index"),
    }

}
