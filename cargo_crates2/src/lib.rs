//Exporting a Convenient Public API with pub use

//In Chapter 7, we covered how to organize our code into modules using the mod keyword, how to make items public using the pub keyword, 
//and how to bring items into a scope with the use keyword. 
//However, the structure that makes sense to you while you’re developing a crate might not be very convenient for your users.

//You might want to organize your structs in a hierarchy containing multiple levels, but then people who want to use a type you’ve defined deep in the hierarchy 
//might have trouble finding out that type exists. 
//They might also be annoyed at having to enter use my_crate::some_module::another_module::UsefulType; rather than use my_crate::UsefulType;.

//The structure of your public API is a major consideration when publishing a crate.

//The good news is that if the structure isn’t convenient for others to use from another library, you don’t have to rearrange your internal organization: instead, 
//you can re-export items to make a public structure that’s different from your private structure by using pub use.
//Re-exporting takes a public item in one location and makes it public in another location, as if it were defined in the other location instead.

//For example, say we made a library named art for modeling artistic concepts. Within this library are two modules: 
//a kinds module containing two enums named PrimaryColor and SecondaryColor and a utils module containing a function named mix

//! # Art
//!
//! A library for modeling artistic concepts.

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        // --snip--
    }
}//Listing 14-3

//Note that the PrimaryColor and SecondaryColor types aren’t listed on the front page, nor is the mix function. We have to click kinds and utils to see them.

//Another crate that depends on this library would need use statements that bring the items from art into scope, specifying the module structure that’s currently defined.

//Listing 14-4 shows an example of a crate that uses the PrimaryColor and mix items from the art crate. check main.rs