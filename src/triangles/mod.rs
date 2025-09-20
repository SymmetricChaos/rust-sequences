pub mod bell;
pub mod bernoulli;
pub mod pascal;

pub use bell::*;
pub use bernoulli::*;
pub use pascal::*;

crate::print_values!(
    print_triangles, formatter "{:?}", sep "\n";
    BellTriangle::new_big(), 0, 10;
    BernoullisTriangle::new_big(), 0, 10;
    PascalsTriangle::new_big(), 0, 10;
);
