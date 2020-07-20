//! # DoublySure
//!
//! > Using types to make sure that you're sure, sure, and doubly sure
//!
//! Users get prompted to make sure they want to perform a destructive action,
//! why shouldn't developers?
//!
//! This crate is composed of only the `AreYouSure` enum, and the `make_sure`
//! macro. Check their individual documentation for more info.
//!
//! ## Usage
//!
//! The core usage is simple, when you encounter an `AreYouSure` type you must
//! answer the question:
//!
//! - Call `.yes_i_am_sure()` to run deferred functions and runwrap the value.
//! - Or call `.no_i_am_not_sure()` to discard, doing nothing.
//!
//! The `make_sure` macro exists to wrap existing values, functions, and code
//! blocks in the `AreYouSure` type.
//!
//! ### Library Usage
//!
//! DoublySure really shines when you use it within the public API of your own
//! libraries as the return type of dangerous functions.
//!
//! ```ignore
//! pub fn dangerous(num: usize) -> AreYouSure<Result<()>> {
//!     // Set things up...
//!
//!     return make_sure!({
//!        // dangerous operation that will not be run
//!        // until the user calls .yes_i_am_sure()
//!     });
//! }
//! ```

mod tests;

/// A type that asks the user the all important question:
/// Are you sure you want to do this?
///
/// The functions on this type are intentionally verbose so that users
/// understand that they are confirming a potentially destructive action.
pub enum AreYouSure<T> {
    /// A value that will be returned when `yes_i_am_sure()` is called.
    Value(T),
    /// A function that was deferred and will be run later when
    /// `yes_i_am_sure()` is called.
    DeferredFunction(Box<dyn Fn() -> T>),
}

impl<T> AreYouSure<T> {
    /// Creates a new `AreYouSure` wrapping around a value.
    /// This is the same as `AreYouSure::Value()`.
    ///
    /// The `make_sure` macro is the preferred way of creating an `AreYouSure`.
    pub fn new(val: T) -> Self {
        Self::Value(val)
    }

    /// You are, in fact, sure that you want to do this.
    ///
    /// Returns the inner value, or runs the deferred function and returns
    /// whatever it does.
    pub fn yes_i_am_sure(self) -> T {
        match self {
            Self::Value(x) => x,
            Self::DeferredFunction(f) => f(),
        }
    }

    /// You're not actually sure you want to do this.
    ///
    /// This will discard the `AreYouSure` dropping it's value and will not run
    /// deferred functions.
    pub fn no_i_am_not_sure(self) {
        // Does nothing, but took ownership of `self`
    }
}

/// Macro to wrap values in an `AreYouSure`. Or in other words, it makes sure of
/// something.
///
/// Creates an `AreYouSure`, possibly deferring execution of a function or block
/// of code.
///
/// ## Variants
/// This macro has three variants, each with their own use cases.
///
/// You can wrap a value or expression directly.
///
/// ```
/// # #[macro_use] extern crate doublysure;
/// # fn main() {
///
/// let sure = make_sure!(4 + 1);
/// let x = sure.yes_i_am_sure();
/// assert_eq!(x, 5);
///
/// # }
/// ```
///
/// You can defer a function. While it looks like `min()` is called here,
/// it will actually be called later when `yes_i_am_sure()` is called.
/// This means that functions with destructive side effects cannot happen until
/// you have confirmed you are sure.
///
/// ```
/// # #[macro_use] extern crate doublysure;
/// # fn main() {
///
/// let sure = make_sure!(std::cmp::min(4, 5));
/// let x = sure.yes_i_am_sure();
/// assert_eq!(x, 4);
///
/// # }
/// ```
///
/// Finally you can defer a block of code for later execution. Keep in mind you
/// still need the parentheses around the block.
/// This form requires the most care to use. Keep in mind as well that the
/// return value of the block is the last expression without a `;` and **not**
/// the `return`.
///
/// ```
/// # #[macro_use] extern crate doublysure;
/// # fn main() {
///
/// let sure = make_sure!({
///   let x = 4;
///   let y = 5;
///   std::cmp::min(x, y)
/// });
/// let x = sure.yes_i_am_sure();
/// assert_eq!(x, 4);
///
/// # }
/// ```
#[macro_export]
macro_rules! make_sure {
    ($name:ident($($arg:expr),*)) => {
        $crate::AreYouSure::DeferredFunction(Box::new(|| $name($($arg),*)))
    };
    ($code:block) => {
        $crate::AreYouSure::DeferredFunction(Box::new(|| { $code }))
    };
    ($value:expr) => {
        $crate::AreYouSure::Value($value)
    };
}
