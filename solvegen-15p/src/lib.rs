mod db;
mod heu;
mod puzzle;
mod rbfs;
mod util;

pub use db::*;
pub use puzzle::*;
pub use rbfs::*;
pub use util::*;

pub(crate) fn pause() {
    use std::io::Read;
    let _ = std::io::stdin().read(&mut [0u8]).unwrap();
}
