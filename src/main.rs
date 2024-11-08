mod allocator;
use allocator::{BasicAllocator, Locked};

#[global_allocator]
static ALLOCATOR: Locked<BasicAllocator> = Locked::new();

fn main() {
    let n = vec![1,2,3];
    println!("{:?}",n);
}
