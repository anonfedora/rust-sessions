// Write a Generic Add, Subtract, Multiply, Divide, Modulo function
// It should take in two parameters
// and return the summation.

// Write a generic function that checks the
// GCD of two given numbers.

// Write a generic Binary Search function
// Start searching from the middle
// If the item at the middle is
// the item you're looking for,
// return the middle index
// If the item at the middle is greater
// than the item you're searching for,
// Set search scope upper bound to be mid - 1
// Similarly, if the item at the middle is
// less than the item you're searching for,
// Set search scope lower bound to be mid + 1
use std::cmp::Ordering;

pub fn binary_search<T>(list: &[T], item: &T) -> Option<usize>
where
    T: Ord,
{
}
