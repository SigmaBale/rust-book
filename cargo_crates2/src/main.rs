use art::kinds::PrimaryColor;
use art::utils::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}//Listing 14-4

//The author of the code in Listing 14-4, which uses the art crate, had to figure out that PrimaryColor is in the kinds module and mix is in the utils module. 
//The module structure of the art crate is more relevant to developers working on the art crate than to developers using the art crate.

//The internal structure that organizes parts of the crate into the kinds module and the utils module doesn’t contain any useful information for someone 
//trying to understand how to use the art crate. 
//Instead, the art crate’s module structure causes confusion because developers have to figure out where to look, 
//and the structure is inconvenient because developers must specify the module names in the use statements.

//To remove the internal organization from the public API, we can modify the art crate code in Listing 14-3 to add pub use statements to re-export 
//the items at the top level, as shown in Listing 14-5:
//check lib.rs

//The API documentation that cargo doc generates for this crate will now list and link re-exports on the front page.
//The art crate users can still see and use the internal structure from Listing 14-3 as demonstrated in Listing 14-4, 
//or they can use the more convenient structure in Listing 14-5, as shown in Listing 14-6:

/*
use art::mix;
use art::PrimaryColor;
*///Listing 14-6

//In cases where there are many nested modules, re-exporting the types at the top level with pub use can make a 
//significant difference in the experience of people who use the crate.

//Creating a useful public API structure is more of an art than a science, and you can iterate to find the API that works best for your users. 
//Choosing pub use gives you flexibility in how you structure your crate internally and decouples that internal structure from what you present to your users. 
//Look at some of the code of crates you’ve installed to see if their internal structure differs from their public API.


//SETTING UP A Crates.io ACCOUNT

//Before you can publish any crates, you need to create an account on crates.io and get an API token. 
//To do so, visit the home page at crates.io and log in via a GitHub account. 
//(The GitHub account is currently a requirement, but the site might support other ways of creating an account in the future.) 
//Once you’re logged in, visit your account settings at https://crates.io/me/ and retrieve your API key. 
//Then run the cargo login command with your API key, like this:

//$ cargo login abcdefghijklmnopqrstuvwxyz012345

//This command will inform Cargo of your API token and store it locally in ~/.cargo/credentials. Note that this token is a secret: do not share it with anyone else. 
//If you do share it with anyone for any reason, you should revoke it and generate a new token on crates.io.


//Adding Metadata to a New Crate

//Now that you have an account, let’s say you have a crate you want to publish. 
//Before publishing, you’ll need to add some metadata to your crate by adding it to the [package] section of the crate’s Cargo.toml file.

//Your crate will need a unique name. While you’re working on a crate locally, you can name a crate whatever you’d like. 
//However, crate names on crates.io are allocated on a first-come, first-served basis. Once a crate name is taken, no one else can publish a crate with that name. 
//Before attempting to publish a crate, search for the name you want to use on the site. 
//If the name has been used by another crate, you will need to find another name and edit the name field in the Cargo.toml file 
//under the [package] section to use the new name for publishing, like so:

/*
[package]
name = "guessing_game"
*/

//Even if you’ve chosen a unique name, when you run cargo publish to publish the crate at this point, you’ll get a warning and then an error:
/*
    $ cargo publish
        Updating crates.io index
    warning: manifest has no description, license, license-file, documentation, homepage or repository.
    See https://doc.rust-lang.org/cargo/reference/manifest.html#package-metadata for more info.
    --snip--
    error: failed to publish to registry at https://crates.io

    Caused by:
    the remote server responded with an error: missing or empty metadata fields: description, license. Please see https://doc.rust-lang.org/cargo/reference/manifest.html for how to upload metadata

*/

//The reason is that you’re missing some crucial information: a description and license are required so people will know what your crate does and under what terms they can use it. 
//To rectify this error, you need to include this information in the Cargo.toml file.

/*
Add a description that is just a sentence or two, because it will appear with your crate in search results. 
For the license field, you need to give a license identifier value. 
The Linux Foundation’s Software Package Data Exchange (SPDX) lists the identifiers you can use for this value. 
For example, to specify that you’ve licensed your crate using the MIT License, add the MIT identifier:

[package]
name = "guessing_game"
license = "MIT"

If you want to use a license that doesn’t appear in the SPDX, you need to place the text of that license in a file, include the file in your project, 
and then use license-file to specify the name of that file instead of using the license key.

Guidance on which license is appropriate for your project is beyond the scope of this book. 
Many people in the Rust community license their projects in the same way as Rust by using a dual license of MIT OR Apache-2.0. 
This practice demonstrates that you can also specify multiple license identifiers separated by OR to have multiple licenses for your project.

With a unique name, the version, your description, and a license added, the Cargo.toml file for a project that is ready to publish might look like this:

[package]
name = "guessing_game"
version = "0.1.0"
edition = "2021"
description = "A fun game where you guess what number the computer has chosen."
license = "MIT OR Apache-2.0"

[dependencies]

Cargo’s documentation describes other metadata you can specify to ensure others can discover and use your crate more easily.
*/



//Publishing to Crates.io

/*

Now that you’ve created an account, saved your API token, chosen a name for your crate, and specified the required metadata, you’re ready to publish! 
Publishing a crate uploads a specific version to crates.io for others to use.

Be careful when publishing a crate because a publish is permanent. The version can never be overwritten, and the code cannot be deleted. 
One major goal of crates.io is to act as a permanent archive of code so that builds of all projects that depend on crates from crates.io will continue to work. 
Allowing version deletions would make fulfilling that goal impossible. However, there is no limit to the number of crate versions you can publish.

Run the cargo publish command again. It should succeed now:

    $ cargo publish
        Updating crates.io index
    Packaging guessing_game v0.1.0 (file:///projects/guessing_game)
    Verifying guessing_game v0.1.0 (file:///projects/guessing_game)
    Compiling guessing_game v0.1.0
    (file:///projects/guessing_game/target/package/guessing_game-0.1.0)
        Finished dev [unoptimized + debuginfo] target(s) in 0.19s
    Uploading guessing_game v0.1.0 (file:///projects/guessing_game)


Congratulations! You’ve now shared your code with the Rust community, and anyone can easily add your crate as a dependency of their project.
*/



//Publishing a New Version of an Existing Crate

/*
When you’ve made changes to your crate and are ready to release a new version, you change the version value specified in your Cargo.toml file and republish. 
Use the Semantic Versioning rules to decide what an appropriate next version number is based on the kinds of changes you’ve made. 
Then run cargo publish to upload the new version.
*/



//Removing Versions from Crates.io with cargo yank

/*
Although you can’t remove previous versions of a crate, you can prevent any future projects from adding them as a new dependency. 
This is useful when a crate version is broken for one reason or another. 
In such situations, Cargo supports yanking a crate version.

Yanking a version prevents new projects from starting to depend on that version while allowing all existing projects that depend on it to 
continue to download and depend on that version. 
Essentially, a yank means that all projects with a Cargo.lock will not break, and any future Cargo.lock files generated will not use the yanked version.

To yank a version of a crate, run cargo yank and specify which version you want to yank:
$ cargo yank --vers 1.0.1

By adding --undo to the command, you can also undo a yank and allow projects to start depending on a version again:
$ cargo yank --vers 1.0.1 --undo

A yank does not delete any code. For example, the yank feature is not intended for deleting accidentally uploaded secrets. 
If that happens, you must reset those secrets immediately.
*/