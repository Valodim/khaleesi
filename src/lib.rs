#[macro_use]
mod macros;

pub mod errors;
pub type KhResult<T> = Result<T,errors::KhError>;

pub mod actions;
pub mod backup;
pub mod khline;
pub mod calendars;
pub mod config;
pub mod cursorfile;
pub mod defaults;
pub mod icalwrap;
pub mod input;
pub mod selectors;
pub mod seqfile;
pub mod utils;
#[cfg(test)]
pub mod testutils;

#[cfg(test)]
pub mod testdata;
extern crate tempfile;
#[cfg(test)]
extern crate assert_fs;
#[cfg(test)]
extern crate predicates;

extern crate atty;
extern crate backtrace;
extern crate chrono;
extern crate fs2;
extern crate itertools;
extern crate libc;
extern crate libical_sys as ical;
extern crate stderrlog;
extern crate uuid;
extern crate walkdir;
extern crate yansi;

#[macro_use]
extern crate serde_derive;
extern crate toml;

#[macro_use]
extern crate log;

#[macro_use]
extern crate indoc;

#[cfg(test)]
#[macro_use]
extern crate lazy_static;
