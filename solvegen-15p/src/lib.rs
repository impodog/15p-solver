pub mod astar;
mod db;
mod heu;
mod puzzle;
pub mod random;
pub mod rbfs;
mod search;
mod util;

pub use db::*;
pub use puzzle::*;
pub use search::*;
pub use util::*;

pub(crate) fn pause() {
    use std::io::Read;
    let _ = std::io::stdin().read(&mut [0u8]).unwrap();
}
