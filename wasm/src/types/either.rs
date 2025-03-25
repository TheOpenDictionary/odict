use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Either<A, B> {
    A(A),
    B(B),
}

#[derive(Serialize, Deserialize)]
pub enum Either3<A, B, C> {
    A(A),
    B(B),
    C(C),
}
