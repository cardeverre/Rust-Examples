Some note making whilst on the road to becoming a Rustacean.

Command to start new project: cargo new project name
Ref:
https://www.rust-lang.org/learn/get-started

Finding new dependancies for a project:
https://crates.io/ 

By default, Rust has a set of items defined in the standard library that it brings into the scope of every program. This set is called the prelude, and you can see everything in it:
https://doc.rust-lang.org/std/prelude/index.html

Note: You won’t just know which traits to use and which methods and functions to call from a crate, so each crate has documentation with instructions for using it. Another neat feature of Cargo is that running the cargo doc --open command will build documentation provided by all of your dependencies locally and open it in your browser. If you’re interested in other functionality in the rand crate, for example, run cargo doc --open and click rand in the sidebar on the left.