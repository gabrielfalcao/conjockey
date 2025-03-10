pub trait LogicalOperations:
    Sized
    + std::ops::BitAnd
    + std::ops::BitAndAssign
    + std::ops::BitOr
    + std::ops::BitOrAssign
    + std::ops::BitXor
    + std::ops::BitXorAssign
    + std::ops::Shl
    + std::ops::ShlAssign
    + std::ops::Shr
    + std::ops::ShrAssign
    + std::ops::Sub
    + std::ops::SubAssign
    + std::cmp::Eq
    + std::cmp::Ord
    + std::cmp::PartialEq
    + std::cmp::PartialOrd
{
}
