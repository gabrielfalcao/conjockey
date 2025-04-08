pub trait AlgebraicOperations:
    Sized
    + std::ops::Add
    + std::ops::AddAssign
    + std::ops::Div
    + std::ops::DivAssign
    + std::ops::Mul
    + std::ops::MulAssign
    + std::ops::Neg
    + std::ops::Rem
    + std::ops::RemAssign
    + std::ops::Sub
    + std::ops::SubAssign
    + std::cmp::Eq
    + std::cmp::Ord
    + std::cmp::PartialEq
    + std::cmp::PartialOrd
{
}
