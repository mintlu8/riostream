//! A humorous attempt at implementing C++'s iostream and its syntax in Rust.
//!
//! Differences from c++:
//!
//! * [`rin`] reads lines instead.
//! * [`endl`] and [`flush`] consumes the stream object to avoid an 'unused operator result' lint from rust.
//!
//! Disclaimer: this is purely for fun, do not use this crate in production.

use std::io::{stdin, stdout, Write, stderr};
use std::str::FromStr;
use std::fmt::Display;
use std::ops::{Shl, Shr};
use std::any::type_name;

/// Emulates the behavior of `cin`.
///
/// Unlike `cin`, this reads and parses a line.
///
/// # Examples
///
/// ```no_run
/// # use riostream::*;
/// let mut a = "".to_owned();
/// let mut b = 42;
/// rin >> &mut a >> &mut b;
/// ```
#[allow(nonstandard_style)]
pub struct rin;

/// Emulates the behavior of `cout`.
/// # Examples
///
/// ```
/// # use riostream::*;
/// rout << 42 << "Hello, World!!" << endl;
/// ```
#[allow(nonstandard_style)]
pub struct rout;

/// Emulates the behavior of `cerr`.
/// # Examples
///
/// ```
/// # use riostream::*;
/// rerr << 42 << "Hello, World!!" << endl;
/// ```
#[allow(nonstandard_style)]
pub struct rerr;

/// prints `'\n'` and flushes when passed to [`rout`] or [`rerr`]
///
/// This also consumes the stream to avoid an unused warning.
#[allow(nonstandard_style)]
pub struct endl;

/// Flushes when passed to [`rout`] or [`rerr`].
///
/// This also consumes the stream to avoid an unused warning.
#[allow(nonstandard_style)]
pub struct flush;

impl<T> Shl<T> for rout where T: Display{
    type Output = rout;

    fn shl(self, rhs: T) -> rout {
        stdout().write(rhs.to_string().as_bytes()).unwrap();
        self
    }
}

impl Shl<endl> for rout where{
    type Output = ();

    fn shl(self, _: endl) -> () {
        stdout().write(b"\n").unwrap();
        stdout().flush().unwrap();
    }
}

impl Shl<flush> for rout where{
    type Output = ();

    fn shl(self, _: flush) -> () {
        stdout().flush().unwrap();
    }
}


impl<T> Shl<T> for rerr where T: Display{
    type Output = rerr;

    fn shl(self, rhs: T) -> rerr {
        stderr().write(rhs.to_string().as_bytes()).unwrap();
        self
    }
}

impl Shl<endl> for rerr where{
    type Output = ();

    fn shl(self, _: endl) -> () {
        stderr().write(b"\n").unwrap();
        stderr().flush().unwrap();
    }
}

impl Shl<flush> for rerr where{
    type Output = ();

    fn shl(self, _: flush) -> () {
        stderr().flush().unwrap();
    }
}

impl<T> Shr<&mut T> for rin where T: FromStr {
    type Output = rin;

    #[allow(unused_results)]
    fn shr(self, rhs: &mut T) -> rin {
        let mut buf = String::new();
        stdin().read_line(&mut buf).unwrap();
        if let Ok(item) = T::from_str(&buf) {
            *rhs = item
        } else {
            panic!("\"{}\" is not a {}", &buf, type_name::<T>())
        };
        self
    }
}

impl Shr<flush> for rin {
    type Output = ();

    #[allow(unused_results)]
    fn shr(self, _: flush) -> () {}
}

#[test]
fn test() {
    rout << 1 << " " << "HelloWorld!" << endl;
}
