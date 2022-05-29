pub mod ex1;
pub mod ex10;
pub mod ex11;
pub mod ex12;
pub mod ex13;
pub mod ex14;
pub mod ex14b;
pub mod ex15;
pub mod ex15b;
pub mod ex15c;
pub mod ex17;
pub mod ex17a;
pub mod ex2;
pub mod ex3;
pub mod ex4;
pub mod ex5;
pub mod ex6;
pub mod ex8;
pub mod ex9;

use mimalloc_rust::*;

#[global_allocator]
static GLOBAL_MIMALLOC: GlobalMiMalloc = GlobalMiMalloc;
