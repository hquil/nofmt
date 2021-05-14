//! This crate provides [`nofmt::pls`][pls], a macro that does its best at preventing a code block from being mangled.
//!
//! ```toml
//! [dependencies]
//! nofmt = "1.0"
//! ```
//!
//! ```rust
//! nofmt::pls! {
//! 	//                                      R      G      B      A
//! 	pub const MAIN_WINDOW_BG:  [f32; 4] = [0.187, 0.187, 0.187, 1.  ];
//! 	pub const DEBUG_WINDOW_BG: [f32; 4] = [0.   , 0.   , 0.   , 0.85];
//! 	pub const INPUT_BOX_BG:    [f32; 4] = [0.011, 0.022, 0.055, 1.  ];
//! }
//! ```
//!
//! ## How to bypass rustfmt 101
//! Executing `cargo fmt` on a crate is likely to violently alter every file, irreversibly. So our only option is to just go with it.
//!
//! - To skip a single item declaration, use `#[rustfmt::skip]`
//! - To skip a block, use [`nofmt::pls!`][pls]
//! - To skip an entire file,  use `#![cfg_attr(rustfmt, rustfmt_skip)]`
//!
//! Keep in mind that opting out of rustfmt can **not** be done through `rustfmt.toml`.  
//! Since both `disable_all_formatting = true` and `ignore = ["/"]` are not stable features.
//!
//! ## Known limitations
//! By itself, this method is not perfect, inside of a [`nofmt::pls`][pls] block:
//! -	each line will get trimmed.
//! -	indenting tabs get replaced by spaces.
//!
//! Note that [format macros](https://github.com/rust-lang/rustfmt/issues/8) for rustfmt is an open issue. So this crate might stop working at any point in time.
//!
//! ## Going all out
//! To bypass the limitations, you can use the `rustfmt::skip` attribute in conjunction with the the macro.
//! ```rust
//! #[rustfmt::skip]	  	nofmt::pls!	  	{
//! 	  	let	absolute_preservation_of_whitespace	=	"achieved";
//!   	}
//! ```
//!
//! [pls]: https://docs.rs/nofmt/1.0/nofmt/macro.pls.html
//!

/// Will do whatever it can to keep your code untouched.
#[macro_export]
macro_rules! pls {
    ($($beautiful_code:tt)*) => { $($beautiful_code)* }
}
