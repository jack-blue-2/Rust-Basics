// how to return a random integer:
use rand; // Add this line at the top of your file to include the rand crate

// the difference between a crate and a trait is that a crate is a package of Rust code,
// while a trait is a collection of methods that define shared behavior for types.
// A trait is a set of methods found in a crate.

// Mark the function as public so you can import it elsewhere
pub fn random_integer(n: i32) -> i32 {
  use rand::Rng; // Import the Rng trait if not already imported
  let mut rng = rand::thread_rng(); // Create a random number generator
  return rng.gen_range(1..=n); // Generate a random integer between 1 and n
}
