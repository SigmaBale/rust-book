mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting; //or use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}

use std::collections::HashMap; //using whole path for structs,enums

use std::fmt::Result;
use std::io::Result as IoResult;

use std::{cmp::Ordering, io};

//use std::io::{self, Write};

use std::collections::*; //all public items defined in the path