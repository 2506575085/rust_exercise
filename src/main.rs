
// use std::borrow::BorrowMut;
// use std::borrow::{Borrow, BorrowMut};
// use std::f32::INFINITY;
// use std::borrow::BorrowMut;
// use std::i128;
// use std::ops::Deref;
// use std::thread::sleep;
// use std::time::Duration;
// use std::collections::HashMap;
// use std::rc::Rc;
// use std::cell::RefCell;

use solutions::Solution;
use solutions::arr2_to_vec2;
use crate::crates::my_default::{ TestDefault, default };
use hello_macro::HelloMacro;

mod crates;

#[derive(HelloMacro)]
struct Pancakes;
#[derive(HelloMacro)]
struct HelloMacroStruct;


fn main() {
    println!("{:#?}", TestDefault{x:1,..default()});
    crates::test_match::test_match();
    Pancakes::hello_macro();
    HelloMacroStruct::hello_macro();
}

