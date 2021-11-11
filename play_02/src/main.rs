/*
From:
https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html?highlight=lifetime#dangling-references

---

This fails with the message...

bbox-2016:play_02 birkin$
bbox-2016:play_02 birkin$ cargo check
    Checking play_02 v0.1.0 (/Users/birkin/Documents/rust_projects/lifetime_experimentation_stuff/lifetime_experimentaion_code/play_02)
error[E0106]: missing lifetime specifier
 --> src/main.rs:5:16
  |
5 | fn dangle() -> &String {
  |                ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
  |
5 | fn dangle() -> &'static String {
  |                ~~~~~~~~

For more information about this error, try `rustc --explain E0106`.
error: could not compile `play_02` due to previous error
bbox-2016:play_02 birkin$

---

The explanation...

... Because s is created inside dangle, when the code of dangle is finished, s will be deallocated.
But we tried to return a reference to it. That means this reference would be pointing to an invalid String.
That’s no good! Rust won’t let us do this. ...
*/


fn main() {
    let reference_to_nothing = dangle();
}


fn dangle() -> &String {            // dangle returns a reference to a String
    let s = String::from("hello");  // s is a new String

    &s                              // we return a reference to the String, s
}                                   // Here, s goes out of scope, and is dropped. Its memory goes away. Danger!
