use std::mem; // Import mem for swap

#[rustfmt::skip]
#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<&str> = None;
    // Use `if let` to safely handle the Some case
    if let Some(value) = my_option {
       println!("{value}");
    }

    let my_arr = &[
        -1, -2, -3, // Add comma
        -4, -5, -6
    ];
    println!("My array! Here it is: {my_arr:?}");

    // Create an empty Vec directly
    let my_empty_vec: Vec<i32> = Vec::new();
    // Note: The original code assigned `()` to my_empty_vec,
    // so the println below won't work as intended with `()`.
    // If you need to print it, keep it as Vec::new() and debug print works.
    println!("This Vec is empty, see? {my_empty_vec:?}");

    let mut value_a = 45;
    let mut value_b = 66;
    // Use std::mem::swap for a correct swap
    mem::swap(&mut value_a, &mut value_b);
    println!("value a: {value_a}; value b: {value_b}"); // Now prints a: 66, b: 45
}