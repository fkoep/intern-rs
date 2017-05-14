#[macro_use]
extern crate lazy_static;
extern crate nodrop;

mod pool;
mod intern;

pub use pool::Pool;
pub use intern::Intern;
