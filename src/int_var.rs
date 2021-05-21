use super::VariableState;
use crusp_core::{Mergeable, Nullable, Subsumed};
#[cfg(feature = "graph")]
use crusp_graph::GraphEvent;

// IntVarBounds<T>    [min;max]
// IntVarBitset<T>    Vec<[lb;ub]>
// IntVarList<T>      Vec<T>
// IntVarIntervals<T> {offset; len; BitSet<>}

// pub use self::bounds::{IntVarBounds, IntVarBoundsArray, IntVarBoundsRefArray};
// pub use self::intervals::{IntVarIntervals, IntVarIntervalsArray, IntVarIntervalsRefArray};
// pub use self::values::{IntVarValues, IntVarValuesArray, IntVarValuesRefArray};
// pub use self::values::{IntVarBitset, IntVarBitsetArray, IntVarBitsetRefArray};

pub use self::values::{IntVarValues, IntVarValuesBuilder};

mod bitset;
mod bounds;
mod intervals;
mod values;

/// Describes the state of a variable after its domain is updated.
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum IntVariableState {
    /// If only the maximal bound of the variable has been updated.
    MaxBoundChange = 0b0000_0011,
    /// If only the minimal bound of the variable has been updated.
    MinBoundChange = 0b0000_0101,
    /// If both bounds of the variable has been updated.
    BoundsChange = 0b0000_0111,
    /// If the domain has been change but not its bounds.
    ValuesChange = 0b0000_1111,
    /// If no change occured.
    NoChange = 0b0000_0000,
    /// When the value has been changed by an universal brancher
    UniversalChange = 0b1110_0000,
    UniversalError = 0b1110_0001,
}

#[cfg(feature = "graph")]
impl GraphEvent for IntVariableState {}
impl Nullable for IntVariableState {
    fn is_null(&self) -> bool {
        *self == IntVariableState::NoChange
    }

    fn null() -> Self {
        IntVariableState::NoChange
    }

    fn nullify(&mut self) -> Self {
        let prev = *self;
        *self = IntVariableState::NoChange;
        prev
    }
}
impl Mergeable for IntVariableState {
    fn merge(&self, rhs: Self) -> Self {
        *self | rhs
    }
}

impl std::ops::BitOr for IntVariableState {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        unsafe {
            let lhs: u8 = std::mem::transmute(self);
            let rhs: u8 = std::mem::transmute(rhs);
            let univ: u8 = std::mem::transmute(IntVariableState::UniversalChange);
            let value: u8 = std::mem::transmute(IntVariableState::ValuesChange);
            let univ_bit = (lhs | rhs) & univ;
            let value_bit = (lhs | rhs) & value;
            let value_mask = (!univ_bit) >> 4;
            let res = univ_bit | (value_bit & value_mask);
            std::mem::transmute(res)
        }
    }
}

impl VariableState for IntVariableState {}

#[cfg(test)]
mod tests {
    #[test]
    fn test_op_or() {
        use super::IntVariableState;
        // no change is neutral
        assert_eq!(
            IntVariableState::NoChange | IntVariableState::MaxBoundChange,
            IntVariableState::MaxBoundChange
        );
        assert_eq!(
            IntVariableState::NoChange | IntVariableState::MinBoundChange,
            IntVariableState::MinBoundChange
        );
        assert_eq!(
            IntVariableState::NoChange | IntVariableState::BoundsChange,
            IntVariableState::BoundsChange
        );
        assert_eq!(
            IntVariableState::NoChange | IntVariableState::ValuesChange,
            IntVariableState::ValuesChange
        );
        assert_eq!(
            IntVariableState::NoChange | IntVariableState::NoChange,
            IntVariableState::NoChange
        );
        assert_eq!(
            IntVariableState::NoChange | IntVariableState::UniversalChange,
            IntVariableState::UniversalChange
        );
        assert_eq!(
            IntVariableState::NoChange | IntVariableState::UniversalError,
            IntVariableState::UniversalError
        );
        // max bounds
        assert_eq!(
            IntVariableState::MaxBoundChange | IntVariableState::MaxBoundChange,
            IntVariableState::MaxBoundChange
        );
        assert_eq!(
            IntVariableState::MaxBoundChange | IntVariableState::MinBoundChange,
            IntVariableState::BoundsChange
        );
        assert_eq!(
            IntVariableState::MaxBoundChange | IntVariableState::BoundsChange,
            IntVariableState::BoundsChange
        );
        assert_eq!(
            IntVariableState::MaxBoundChange | IntVariableState::ValuesChange,
            IntVariableState::ValuesChange
        );
        assert_eq!(
            IntVariableState::MaxBoundChange | IntVariableState::NoChange,
            IntVariableState::MaxBoundChange
        );
        assert_eq!(
            IntVariableState::MaxBoundChange | IntVariableState::UniversalChange,
            IntVariableState::UniversalError
        );
        assert_eq!(
            IntVariableState::MaxBoundChange | IntVariableState::UniversalError,
            IntVariableState::UniversalError
        );
        // min bounds
        assert_eq!(
            IntVariableState::MinBoundChange | IntVariableState::MaxBoundChange,
            IntVariableState::BoundsChange
        );
        assert_eq!(
            IntVariableState::MinBoundChange | IntVariableState::MinBoundChange,
            IntVariableState::MinBoundChange
        );
        assert_eq!(
            IntVariableState::MinBoundChange | IntVariableState::BoundsChange,
            IntVariableState::BoundsChange
        );
        assert_eq!(
            IntVariableState::MinBoundChange | IntVariableState::ValuesChange,
            IntVariableState::ValuesChange
        );
        assert_eq!(
            IntVariableState::MinBoundChange | IntVariableState::NoChange,
            IntVariableState::MinBoundChange
        );
        assert_eq!(
            IntVariableState::MinBoundChange | IntVariableState::UniversalChange,
            IntVariableState::UniversalError
        );
        assert_eq!(
            IntVariableState::MinBoundChange | IntVariableState::UniversalError,
            IntVariableState::UniversalError
        );
        // bounds
        assert_eq!(
            IntVariableState::BoundsChange | IntVariableState::MaxBoundChange,
            IntVariableState::BoundsChange
        );
        assert_eq!(
            IntVariableState::BoundsChange | IntVariableState::MinBoundChange,
            IntVariableState::BoundsChange
        );
        assert_eq!(
            IntVariableState::BoundsChange | IntVariableState::BoundsChange,
            IntVariableState::BoundsChange
        );
        assert_eq!(
            IntVariableState::BoundsChange | IntVariableState::ValuesChange,
            IntVariableState::ValuesChange
        );
        assert_eq!(
            IntVariableState::BoundsChange | IntVariableState::NoChange,
            IntVariableState::BoundsChange
        );
        assert_eq!(
            IntVariableState::BoundsChange | IntVariableState::UniversalChange,
            IntVariableState::UniversalError
        );
        assert_eq!(
            IntVariableState::BoundsChange | IntVariableState::UniversalError,
            IntVariableState::UniversalError
        );
        // values
        assert_eq!(
            IntVariableState::ValuesChange | IntVariableState::MaxBoundChange,
            IntVariableState::ValuesChange
        );
        assert_eq!(
            IntVariableState::ValuesChange | IntVariableState::MinBoundChange,
            IntVariableState::ValuesChange
        );
        assert_eq!(
            IntVariableState::ValuesChange | IntVariableState::BoundsChange,
            IntVariableState::ValuesChange
        );
        assert_eq!(
            IntVariableState::ValuesChange | IntVariableState::ValuesChange,
            IntVariableState::ValuesChange
        );
        assert_eq!(
            IntVariableState::ValuesChange | IntVariableState::NoChange,
            IntVariableState::ValuesChange
        );
        assert_eq!(
            IntVariableState::ValuesChange | IntVariableState::UniversalChange,
            IntVariableState::UniversalError
        );
        assert_eq!(
            IntVariableState::ValuesChange | IntVariableState::UniversalError,
            IntVariableState::UniversalError
        );
        // universal
        assert_eq!(
            IntVariableState::UniversalChange | IntVariableState::MaxBoundChange,
            IntVariableState::UniversalError
        );
        assert_eq!(
            IntVariableState::UniversalChange | IntVariableState::MinBoundChange,
            IntVariableState::UniversalError
        );
        assert_eq!(
            IntVariableState::UniversalChange | IntVariableState::BoundsChange,
            IntVariableState::UniversalError
        );
        assert_eq!(
            IntVariableState::UniversalChange | IntVariableState::ValuesChange,
            IntVariableState::UniversalError
        );
        assert_eq!(
            IntVariableState::UniversalChange | IntVariableState::NoChange,
            IntVariableState::UniversalChange
        );
        assert_eq!(
            IntVariableState::UniversalChange | IntVariableState::UniversalChange,
            IntVariableState::UniversalChange
        );
        assert_eq!(
            IntVariableState::UniversalChange | IntVariableState::UniversalError,
            IntVariableState::UniversalError
        );
        // universal error
        assert_eq!(
            IntVariableState::UniversalError | IntVariableState::MaxBoundChange,
            IntVariableState::UniversalError
        );
        assert_eq!(
            IntVariableState::UniversalError | IntVariableState::MinBoundChange,
            IntVariableState::UniversalError
        );
        assert_eq!(
            IntVariableState::UniversalError | IntVariableState::BoundsChange,
            IntVariableState::UniversalError
        );
        assert_eq!(
            IntVariableState::UniversalError | IntVariableState::ValuesChange,
            IntVariableState::UniversalError
        );
        assert_eq!(
            IntVariableState::UniversalError | IntVariableState::NoChange,
            IntVariableState::UniversalError
        );
        assert_eq!(
            IntVariableState::UniversalError | IntVariableState::UniversalChange,
            IntVariableState::UniversalError
        );
        assert_eq!(
            IntVariableState::UniversalError | IntVariableState::UniversalError,
            IntVariableState::UniversalError
        );
    }
}

impl Subsumed for IntVariableState {
    /// # Subsomption relations
    /// * `MaxBoundChange` subsumed `BoundsChange`
    /// * `MinBoundChange` subsumed `BoundsChange`
    /// * `BoundsChange` subsumed `ValuesChange`
    /// * `ValuesChange` subsumed `NoChange`
    fn is_subsumed_under(&self, val: &Self) -> bool {
        // not correct yet
        // (make_bitflags!(self) & make_bitflags!(val)).contains(make_bitflags!(self))
        match *self {
            IntVariableState::MaxBoundChange => *val == IntVariableState::MaxBoundChange,
            IntVariableState::MinBoundChange => *val == IntVariableState::MinBoundChange,
            IntVariableState::BoundsChange => {
                *val != IntVariableState::ValuesChange && *val != IntVariableState::NoChange
            }
            IntVariableState::ValuesChange => *val != IntVariableState::NoChange,
            IntVariableState::NoChange => true,
            _ => false,
        }
    }
}
