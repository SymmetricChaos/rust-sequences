pub mod bell;
pub mod bernoulli;

pub use bell::*;
pub use bernoulli::*;

crate::print_values!(
    print_triangles, formatter "{:?}", sep "\n";
    BellTriangle::new_big(), 0, 10;
    BernoullisTriangle::new_big(), 0, 10;
);
