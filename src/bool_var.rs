use crate::domains::{AssignableDomain, EqualDomain, FiniteDomain, IterableDomain, PrunableDomain};
use crate::int_var::IntVariableState;
use crate::{Variable, VariableError};

#[derive(Clone, Debug, Eq, PartialEq)]
enum BoolDomain {
    True,
    False,
    Both,
    None,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BoolVar {
    domain: BoolDomain,
    state: IntVariableState,
}

impl BoolVar {
    pub fn new() -> Option<BoolVar> {
        Some(BoolVar {
            domain: BoolDomain::Both,
            state: IntVariableState::NoChange,
        })
    }
}

impl IterableDomain<bool> for BoolVar {
    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = &bool> + 'a> {
        unimplemented!()
    }
}

impl AssignableDomain<bool, IntVariableState> for BoolVar {
    fn set_value(&mut self, value: bool) -> Result<IntVariableState, VariableError> {
        let value = match self.domain {
            BoolDomain::Both => value,
            BoolDomain::True if value => {
                return Ok(IntVariableState::NoChange);
            }
            BoolDomain::False if !value => {
                return Ok(IntVariableState::NoChange);
            }
            _ => {
                self.domain = BoolDomain::None;
                return Err(VariableError::DomainWipeout);
            }
        };
        self.domain = if value {
            BoolDomain::True
        } else {
            BoolDomain::False
        };
        Ok(IntVariableState::BoundsChange)
    }
}

impl Variable<bool> for BoolVar {
    fn is_affected(&self) -> bool {
        self.domain == BoolDomain::True || self.domain == BoolDomain::False
    }

    fn value(&self) -> Option<&bool> {
        match self.domain {
            BoolDomain::True => Some(&true),
            BoolDomain::False => Some(&false),
            _ => None,
        }
    }
}

impl FiniteDomain<bool> for BoolVar {
    fn size(&self) -> usize {
        match self.domain {
            BoolDomain::True => 1,
            BoolDomain::False => 1,
            BoolDomain::Both => 2,
            _ => 0,
        }
    }
}

impl EqualDomain<bool, IntVariableState> for BoolVar {
    fn equal(
        &mut self,
        _value: &mut Self,
    ) -> Result<(IntVariableState, IntVariableState), VariableError> {
        unimplemented!()
    }

    fn not_equal(
        &mut self,
        _value: &mut BoolVar,
    ) -> Result<(IntVariableState, IntVariableState), VariableError> {
        unimplemented!()
    }
}

impl PrunableDomain<bool, IntVariableState> for BoolVar {
    fn in_values<Values>(&mut self, _values: Values) -> Result<IntVariableState, VariableError>
    where
        Values: IntoIterator<Item = bool>,
    {
        unimplemented!()
    }

    #[allow(unused)]
    fn remove_value(&mut self, value: bool) -> Result<IntVariableState, VariableError> {
        unimplemented!()
    }

    #[allow(unused)]
    fn remove_if<Predicate>(
        &mut self,
        mut pred: Predicate,
    ) -> Result<IntVariableState, VariableError>
    where
        Predicate: FnMut(&bool) -> bool,
    {
        unimplemented!()
    }

    #[allow(unused)]
    fn retains_if<Predicate>(
        &mut self,
        mut pred: Predicate,
    ) -> Result<IntVariableState, VariableError>
    where
        Predicate: FnMut(&bool) -> bool,
    {
        unimplemented!()
    }
}
