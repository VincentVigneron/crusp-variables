use super::IntVariableState;
use crate::domains::{
    AssignableDomain, EqualDomain, FiniteDomain, OrderedPrunableDomain,
    FromRangeDomain, FromValuesDomain, IterableDomain, OrderedDomain, PrunableDomain,
};
#[cfg(feature = "observer")]
use crate::domains::{
    AssignableDomainObserver, EqualDomainObserver,
    OrderedDomainObserver,
    OrderedPrunableDomainObserver, PrunableDomainObserver,
};
#[cfg(feature = "observer")]
use crate::{CruspVariable, VariableObserver};
use crate::{Variable, VariableError};
use crusp_core::VariableId;
use crusp_core::{unwrap_first, unwrap_last};
use num::One;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IntVarValues<T>
where
    T: Copy + Clone + Eq + PartialEq + Ord + PartialOrd,
{
    domain: Vec<T>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CruspIntVarValues<T>
where
    T: Copy + Clone + Eq + PartialEq + Ord + PartialOrd,
{
    id: VariableId,
    domain: Vec<T>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IntVarValuesBuilder<T>
where
    T: Copy + Clone + Eq + PartialEq + Ord + PartialOrd,
{
    domain: Vec<T>,
}

impl<T> IntVarValuesBuilder<T>
where
    T: Copy + Clone + Eq + PartialEq + Ord + PartialOrd,
{
    pub fn try_new<U>(min: U, max: U) -> Option<IntVarValuesBuilder<U>>
    where
        U: Copy + Clone + Eq + PartialEq + Ord + PartialOrd + std::ops::Add<Output = U> + One,
    {
        if min > max {
            None
        } else {
            let one = U::one();
            let mut val = min;
            let mut domain = vec![];
            while val < max + one {
                domain.push(val);
                val = val + one;
            }
            Some(IntVarValuesBuilder::<U> { domain })
        }
    }

    pub fn finalize(self) -> IntVarValues<T> {
        IntVarValues {
            domain: self.domain,
        }
    }
}

unsafe impl<T> Sync for IntVarValues<T> where T: Copy + Clone + Eq + PartialEq + Ord + PartialOrd {}
unsafe impl<T> Send for IntVarValues<T> where T: Copy + Clone + Eq + PartialEq + Ord + PartialOrd {}

impl<T> IntVarValues<T>
where
    T: Copy + Clone + Eq + PartialEq + Ord + PartialOrd,
{
    pub fn try_new<U>(min: U, max: U) -> Option<IntVarValues<U>>
    where
        U: Copy + Clone + Eq + PartialEq + Ord + PartialOrd + std::ops::Add<Output = U> + One,
    {
        if min > max {
            None
        } else {
            let one = U::one();
            let mut val = min;
            let mut domain = vec![];
            while val < max + one {
                domain.push(val);
                val = val + one;
            }
            Some(IntVarValues { domain })
        }
    }

    fn invalidate(&mut self) {
        self.domain.clear();
    }

    fn domain_change(
        &mut self,
        prev_min: T,
        prev_max: T,
        prev_size: usize,
    ) -> Result<IntVariableState, VariableError> {
        if self.domain.is_empty() {
            self.invalidate();
            Err(VariableError::DomainWipeout)
        } else if self.size() == prev_size {
            Ok(IntVariableState::NoChange)
        } else if *self.unchecked_min() != prev_min || *self.unchecked_max() != prev_max {
            Ok(IntVariableState::BoundsChange)
        } else {
            Ok(IntVariableState::ValuesChange)
        }
    }
}

#[cfg(feature = "observer")]
impl<T> CruspIntVarValues<T>
where
    T: Copy + Clone + Eq + PartialEq + Ord + PartialOrd,
{
    /*pub fn try_new<U>(min: U, max: U) -> Option<IntVarValues<U>>
        where
            U: Copy + Clone + Eq + PartialEq + Ord + PartialOrd + std::ops::Add<Output = U> + One,
        {
            if min > max {
                None
            } else {
                let one = U::one();
                let mut val = min;
                let mut domain = vec![];
                while val < max + one {
                    domain.push(val);
                    val = val + one;
                }
                Some(IntVarValues { domain })
            }
        }
    */
    fn invalidate(&mut self) {
        self.domain.clear();
    }

    fn domain_change<Observer>(
        &mut self,
        observer: &mut Observer,
        prev_min: T,
        prev_max: T,
        prev_size: usize,
    ) -> Result<IntVariableState, VariableError>
    where
        Observer: VariableObserver<IntVariableState>,
    {
        if self.domain.is_empty() {
            self.invalidate();
            observer.push_error(self.id, VariableError::DomainWipeout)
        } else if self.size() == prev_size {
            Ok(IntVariableState::NoChange)
        } else if *self.unchecked_min() != prev_min || *self.unchecked_max() != prev_max {
            observer.push_change(self.id, IntVariableState::BoundsChange)
        } else {
            observer.push_change(self.id, IntVariableState::ValuesChange)
        }
    }
}

impl<T> IterableDomain<T> for IntVarValues<T>
where
    T: Copy + Clone + Eq + PartialEq + Ord + PartialOrd,
{
    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = &T> + 'a> {
        Box::new(self.domain.iter())
    }
}

impl<T> FromRangeDomain<T> for IntVarValues<T>
where
    T: Copy + Clone + Eq + PartialEq + Ord + PartialOrd + std::ops::Add<Output = T> + One,
{
    fn new_from_range(min: T, max: T) -> Option<IntVarValues<T>> {
        if min > max {
            None
        } else {
            let one = T::one();
            let mut val = min;
            let mut domain = vec![];
            while val < max + one {
                domain.push(val);
                val = val + one;
            }
            Some(IntVarValues { domain })
        }
    }
}

impl<T> FromValuesDomain<T> for IntVarValues<T>
where
    T: Copy + Clone + Eq + PartialEq + Ord + PartialOrd,
{
    fn new_from_values<Values>(values: Values) -> Option<IntVarValues<T>>
    where
        Values: IntoIterator<Item = T>,
    {
        let mut domain = values.into_iter().collect::<Vec<_>>();
        domain.sort();
        domain.dedup();
        if domain.is_empty() {
            None
        } else {
            Some(IntVarValues { domain })
        }
    }
}

impl<T> AssignableDomain<T, IntVariableState> for IntVarValues<T>
where
    T: Copy + Clone + Eq + PartialEq + Ord + PartialOrd,
{
    fn set_value(&mut self, value: T) -> Result<IntVariableState, VariableError> {
        if *self.unchecked_min() > value || *self.unchecked_max() < value {
            //self.invalidate();
            return Err(VariableError::DomainWipeout);
        }
        let var_value = self.value();
        match var_value {
            Some(var_value) if *var_value == value => Ok(IntVariableState::NoChange),
            _ => {
                let found_value = self.domain.binary_search(&value);
                match found_value {
                    Ok(_) => {
                        self.domain = vec![value];
                        Ok(IntVariableState::BoundsChange)
                    }
                    _ => {
                        self.invalidate();
                        Err(VariableError::DomainWipeout)
                    }
                }
            }
        }
    }
}

#[cfg(feature = "observer")]
impl<T> AssignableDomainObserver<T, IntVariableState> for CruspIntVarValues<T>
where
    T: Copy + Clone + Eq + PartialEq + Ord + PartialOrd,
{
    fn set_value<Observer>(
        &mut self,
        observer: &mut Observer,
        value: T,
    ) -> Result<IntVariableState, VariableError>
    where
        Observer: VariableObserver<IntVariableState>,
    {
        if *self.unchecked_min() > value || *self.unchecked_max() < value {
            //self.invalidate();
            return observer.push_error(self.id, VariableError::DomainWipeout)
        }
        let var_value = self.value();
        match var_value {
            Some(var_value) if *var_value == value => Ok(IntVariableState::NoChange),
            _ => {
                let found_value = self.domain.binary_search(&value);
                match found_value {
                    Ok(_) => {
                        self.domain = vec![value];
                        observer.push_change(self.id, IntVariableState::BoundsChange)
                    }
                    _ => {
                        self.invalidate();
                        observer.push_error(self.id, VariableError::DomainWipeout)
                    }
                }
            }
        }
    }
}

impl<T> Variable<T> for IntVarValues<T>
where
    T: Copy + Clone + Eq + PartialEq + Ord + PartialOrd,
{
    fn is_affected(&self) -> bool {
        self.domain.len() == 1
    }

    fn value(&self) -> Option<&T> {
        if self.min() == self.max() {
            self.min()
        } else {
            None
        }
    }
}

#[cfg(feature = "observer")]
impl<T> Variable<T> for CruspIntVarValues<T>
where
    T: Copy + Clone + Eq + PartialEq + Ord + PartialOrd,
{
    fn is_affected(&self) -> bool {
        self.domain.len() == 1
    }

    fn value(&self) -> Option<&T> {
        if self.min() == self.max() {
            self.min()
        } else {
            None
        }
    }
}

#[cfg(feature = "observer")]
impl<T> CruspVariable<T> for CruspIntVarValues<T>
where
    T: Copy + Clone + Eq + PartialEq + Ord + PartialOrd,
{
    fn id(&self) -> VariableId {
        self.id
    }
}

impl<T> FiniteDomain<T> for IntVarValues<T>
where
    T: Copy + Clone + Eq + PartialEq + Ord + PartialOrd,
{
    fn size(&self) -> usize {
        self.domain.len()
    }
}

#[cfg(feature = "observer")]
impl<T> FiniteDomain<T> for CruspIntVarValues<T>
where
    T: Copy + Clone + Eq + PartialEq + Ord + PartialOrd,
{
    fn size(&self) -> usize {
        self.domain.len()
    }
}

impl<T> OrderedDomain<T, IntVariableState> for IntVarValues<T>
where
    T: Copy + Clone + Eq + PartialEq + Ord + PartialOrd,
{
    fn min(&self) -> Option<&T> {
        self.domain.first()
    }
    fn max(&self) -> Option<&T> {
        self.domain.last()
    }

    fn strict_upperbound(&mut self, ub: &T) -> Result<IntVariableState, VariableError> {
        if *self.unchecked_max() < *ub {
            Ok(IntVariableState::NoChange)
        } else if *self.unchecked_min() >= *ub {
            Err(VariableError::DomainWipeout)
        } else {
            let index = self.domain.iter().rposition(|&val| val < *ub).unwrap();
            self.domain.truncate(index + 1);
            Ok(IntVariableState::BoundsChange)
        }
    }

    fn weak_upperbound(&mut self, ub: &T) -> Result<IntVariableState, VariableError> {
        if *self.unchecked_max() <= *ub {
            Ok(IntVariableState::NoChange)
        } else if *self.unchecked_min() > *ub {
            Err(VariableError::DomainWipeout)
        } else {
            let index = self.domain.iter().rposition(|&val| val <= *ub).unwrap();
            self.domain.truncate(index + 1);
            Ok(IntVariableState::BoundsChange)
        }
    }

    fn strict_lowerbound(&mut self, lb: &T) -> Result<IntVariableState, VariableError> {
        if *self.unchecked_min() > *lb {
            Ok(IntVariableState::NoChange)
        } else if *self.unchecked_max() <= *lb {
            Err(VariableError::DomainWipeout)
        } else {
            let index = self.domain.iter().position(|&val| val > *lb).unwrap();
            self.domain.drain(0..index);
            Ok(IntVariableState::BoundsChange)
        }
    }

    fn weak_lowerbound(&mut self, lb: &T) -> Result<IntVariableState, VariableError> {
        if *self.unchecked_min() >= *lb {
            Ok(IntVariableState::NoChange)
        } else if *self.unchecked_max() < *lb {
            Err(VariableError::DomainWipeout)
        } else {
            let index = self.domain.iter().position(|&val| val >= *lb).unwrap();
            self.domain.drain(0..index);
            Ok(IntVariableState::BoundsChange)
        }
    }
}

#[cfg(feature = "observer")]
impl<T> OrderedDomainObserver<T, IntVariableState> for CruspIntVarValues<T>
where
    T: Copy + Clone + Eq + PartialEq + Ord + PartialOrd,
{
    fn min(&self) -> Option<&T> {
        self.domain.first()
    }
    fn max(&self) -> Option<&T> {
        self.domain.last()
    }

    fn strict_upperbound<Observer>(
        &mut self,
        observer: &mut Observer,
        ub: &T,
    ) -> Result<IntVariableState, VariableError>
    where
        Observer: VariableObserver<IntVariableState>,
    {
        if *self.unchecked_max() < *ub {
            Ok(IntVariableState::NoChange)
        } else if *self.unchecked_min() >= *ub {
            observer.push_error(self.id, VariableError::DomainWipeout)
        } else {
            let index = self.domain.iter().rposition(|&val| val < *ub).unwrap();
            self.domain.truncate(index + 1);
            observer.push_change(self.id, IntVariableState::BoundsChange)
        }
    }

    fn weak_upperbound<Observer>(
        &mut self,
        observer: &mut Observer,
        ub: &T,
    ) -> Result<IntVariableState, VariableError>
    where
        Observer: VariableObserver<IntVariableState>,
    {
        if *self.unchecked_max() <= *ub {
            Ok(IntVariableState::NoChange)
        } else if *self.unchecked_min() > *ub {
            observer.push_error(self.id, VariableError::DomainWipeout)
        } else {
            let index = self.domain.iter().rposition(|&val| val <= *ub).unwrap();
            self.domain.truncate(index + 1);
            observer.push_change(self.id,  IntVariableState::BoundsChange)
        }
    }

    fn strict_lowerbound<Observer>(
        &mut self,
        observer: &mut Observer,
        lb: &T,
    ) -> Result<IntVariableState, VariableError>
    where
        Observer: VariableObserver<IntVariableState>,
    {
        if *self.unchecked_min() > *lb {
            Ok(IntVariableState::NoChange)
        } else if *self.unchecked_max() <= *lb {
            observer.push_error(self.id, VariableError::DomainWipeout)
        } else {
            let index = self.domain.iter().position(|&val| val > *lb).unwrap();
            self.domain.drain(0..index);
            observer.push_change(self.id,  IntVariableState::BoundsChange)
        }
    }

    fn weak_lowerbound<Observer>(
        &mut self,
        observer: &mut Observer,
        lb: &T,
    ) -> Result<IntVariableState, VariableError>
    where
        Observer: VariableObserver<IntVariableState>,
    {
        if *self.unchecked_min() >= *lb {
            Ok(IntVariableState::NoChange)
        } else if *self.unchecked_max() < *lb {
            observer.push_error(self.id, VariableError::DomainWipeout)
        } else {
            let index = self.domain.iter().position(|&val| val >= *lb).unwrap();
            self.domain.drain(0..index);
            observer.push_change(self.id,  IntVariableState::BoundsChange)
        }
    }
}

impl<T> EqualDomain<T, IntVariableState> for IntVarValues<T>
where
    T: Copy + Clone + Eq + PartialEq + Ord + PartialOrd,
{
    // Distinction between ValuesChange and BoundsChange
    fn equal(
        &mut self,
        value: &mut Self,
    ) -> Result<(IntVariableState, IntVariableState), VariableError> {
        use std::collections::BTreeSet;
        let s1: BTreeSet<_> = self.iter().copied().collect();
        let s2: BTreeSet<_> = value.iter().copied().collect();
        let domain: Vec<_> = s1.intersection(&s2).copied().collect();

        if domain.is_empty() {
            self.invalidate();
            value.invalidate();
            return Err(VariableError::DomainWipeout);
        }
        let (ok_self, ok_value) = {
            let check_change = |var: &mut IntVarValues<T>| {
                if var.size() == domain.len() {
                    IntVariableState::NoChange
                } else if *var.unchecked_min() != unwrap_first!(domain)
                    || *var.unchecked_max() != unwrap_last!(domain)
                {
                    IntVariableState::BoundsChange
                } else {
                    IntVariableState::ValuesChange
                }
            };
            (check_change(self), check_change(value))
        };

        self.domain = domain.clone();
        value.domain = domain;
        Ok((ok_self, ok_value))
    }
    fn not_equal(
        &mut self,
        value: &mut IntVarValues<T>,
    ) -> Result<(IntVariableState, IntVariableState), VariableError> {
        match self.value() {
            Some(val) => {
                let ok_value = value.remove_value(*val)?;
                Ok((IntVariableState::NoChange, ok_value))
            }
            _ => match value.value() {
                Some(val) => {
                    let ok_self = self.remove_value(*val)?;
                    Ok((ok_self, IntVariableState::NoChange))
                }
                _ => Ok((IntVariableState::NoChange, IntVariableState::NoChange)),
            },
        }
    }
}

#[cfg(feature = "observer")]
impl<T> EqualDomainObserver<T, IntVariableState> for CruspIntVarValues<T>
where
    T: Copy + Clone + Eq + PartialEq + Ord + PartialOrd,
{
    // Distinction between ValuesChange and BoundsChange
    fn equal<Observer>(
        &mut self,
        observer: &mut Observer,
        value: &mut Self,
    ) -> Result<(IntVariableState, IntVariableState), VariableError>
    where
        Observer: VariableObserver<IntVariableState>,
    {
        use std::collections::BTreeSet;
        let s1: BTreeSet<_> = self.domain.iter().copied().collect();
        let s2: BTreeSet<_> = value.domain.iter().copied().collect();
        let domain: Vec<_> = s1.intersection(&s2).copied().collect();

        if domain.is_empty() {
            self.invalidate();
            value.invalidate();
            let _err = observer.push_error(self.id, VariableError::DomainWipeout);
            let _err = observer.push_error(value.id, VariableError::DomainWipeout);
            return Err(VariableError::DomainWipeout);
        }
        let (ok_self, ok_value) = {
            let check_change = |var: &mut CruspIntVarValues<T>| {
                if var.size() == domain.len() {
                    IntVariableState::NoChange
                } else if *var.unchecked_min() != unwrap_first!(domain)
                    || *var.unchecked_max() != unwrap_last!(domain)
                {
                    IntVariableState::BoundsChange
                } else {
                    IntVariableState::ValuesChange
                }
            };
            (check_change(self), check_change(value))
        };

        if ok_self != IntVariableState::NoChange {
            let _change = observer.push_change(self.id, ok_self);
        }

        if ok_value != IntVariableState::NoChange {
            let _change = observer.push_change(value.id, ok_value);
        }
        self.domain = domain.clone();
        value.domain = domain;
        Ok((ok_self, ok_value))
    }
    fn not_equal<Observer>(
        &mut self,
        observer: &mut Observer,
        value: &mut CruspIntVarValues<T>,
    ) -> Result<(IntVariableState, IntVariableState), VariableError>
    where
        Observer: VariableObserver<IntVariableState>,
    {
        match self.value() {
            Some(val) => {
                let ok_value = value.remove_value(observer, *val)?;
                Ok((IntVariableState::NoChange, ok_value))
            }
            _ => match value.value() {
                Some(val) => {
                    let ok_self = self.remove_value(observer, *val)?;
                    let _change = observer.push_change(self.id, ok_self);
                    Ok((ok_self, IntVariableState::NoChange))
                }
                _ => Ok((IntVariableState::NoChange, IntVariableState::NoChange)),
            },
        }
    }
}

impl<T> PrunableDomain<T, IntVariableState> for IntVarValues<T>
where
    T: Copy + Clone + Eq + PartialEq + Ord + PartialOrd,
{
    fn in_values<Values>(&mut self, values: Values) -> Result<IntVariableState, VariableError>
    where
        Values: IntoIterator<Item = T>,
    {
        let values: Vec<_> = values.into_iter().collect();
        let mut values: Vec<_> = values.into_iter().collect();
        values.sort();
        self.in_sorted_values(values.into_iter())
    }

    // check change function (equality, bounds, values, nochange...)
    fn remove_value(&mut self, value: T) -> Result<IntVariableState, VariableError> {
        if *self.unchecked_min() > value && *self.unchecked_max() < value {
            return Ok(IntVariableState::NoChange);
        }
        let (min, max) = (self.min().copied(), self.max().copied());
        let found_value = self.domain.binary_search(&value);
        match found_value {
            Ok(index) => {
                self.domain.remove(index);
                if self.size() == 0 {
                    Err(VariableError::DomainWipeout)
                } else if self.min().copied() != min || self.max().copied() != max {
                    Ok(IntVariableState::BoundsChange)
                } else {
                    Ok(IntVariableState::ValuesChange)
                }
            }
            _ => Ok(IntVariableState::NoChange),
        }
    }

    fn remove_if<Predicate>(
        &mut self,
        mut pred: Predicate,
    ) -> Result<IntVariableState, VariableError>
    where
        Predicate: FnMut(&T) -> bool,
    {
        let (min, max, size) = (*self.unchecked_min(), *self.unchecked_max(), self.size());
        self.domain.retain(|v| !pred(v));
        self.domain_change(min, max, size)
    }

    fn retains_if<Predicate>(
        &mut self,
        mut pred: Predicate,
    ) -> Result<IntVariableState, VariableError>
    where
        Predicate: FnMut(&T) -> bool,
    {
        let (min, max, size) = (*self.unchecked_min(), *self.unchecked_max(), self.size());
        self.domain.retain(|v| pred(v));
        self.domain_change(min, max, size)
    }
}

#[cfg(feature = "observer")]
impl<T> PrunableDomainObserver<T, IntVariableState> for CruspIntVarValues<T>
where
    T: Copy + Clone + Eq + PartialEq + Ord + PartialOrd,
{
    fn in_values<Observer, Values>(
        &mut self,
        observer: &mut Observer,
        values: Values,
    ) -> Result<IntVariableState, VariableError>
    where
        Observer: VariableObserver<IntVariableState>,
        Values: IntoIterator<Item = T>,
    {
        let values: Vec<_> = values.into_iter().collect();
        let mut values: Vec<_> = values.into_iter().collect();
        values.sort();
        self.in_sorted_values(observer, values.into_iter())
    }

    // check change function (equality, bounds, values, nochange...)
    fn remove_value<Observer>(
        &mut self,
        observer: &mut Observer,
        value: T,
    ) -> Result<IntVariableState, VariableError>
    where
        Observer: VariableObserver<IntVariableState>,
    {
        if *self.unchecked_min() > value && *self.unchecked_max() < value {
            return Ok(IntVariableState::NoChange);
        }
        let (min, max) = (self.min().copied(), self.max().copied());
        let found_value = self.domain.binary_search(&value);
        match found_value {
            Ok(index) => {
                self.domain.remove(index);
                if self.size() == 0 {
                    observer.push_error(self.id, VariableError::DomainWipeout)
                } else if self.min().copied() != min || self.max().copied() != max {
                    observer.push_change(self.id, IntVariableState::BoundsChange)
                } else {
                    observer.push_change(self.id, IntVariableState::ValuesChange)
                }
            }
            _ => Ok(IntVariableState::NoChange),
        }
    }

    fn remove_if<Observer, Predicate>(
        &mut self,
        observer: &mut Observer,
        mut pred: Predicate,
    ) -> Result<IntVariableState, VariableError>
    where
        Observer: VariableObserver<IntVariableState>,
        Predicate: FnMut(&T) -> bool,
    {
        let (min, max, size) = (*self.unchecked_min(), *self.unchecked_max(), self.size());
        self.domain.retain(|v| !pred(v));
        self.domain_change(observer, min, max, size)
    }

    fn retains_if<Observer, Predicate>(
        &mut self,
        observer: &mut Observer,
        mut pred: Predicate,
    ) -> Result<IntVariableState, VariableError>
    where
        Observer: VariableObserver<IntVariableState>,
        Predicate: FnMut(&T) -> bool,
    {
        let (min, max, size) = (*self.unchecked_min(), *self.unchecked_max(), self.size());
        self.domain.retain(|v| pred(v));
        self.domain_change(observer, min, max, size)
    }
}

impl<T> OrderedPrunableDomain<T, IntVariableState> for IntVarValues<T>
where
    T: Copy + Clone + Eq + PartialEq + Ord + PartialOrd,
{
    // Change to non-naive implementation
    fn in_sorted_values<Values>(
        &mut self,
        values: Values,
    ) -> Result<IntVariableState, VariableError>
    where
        Values: IntoIterator<Item = T>,
    {
        use std::collections::BTreeSet;
        let s1: BTreeSet<_> = self.iter().copied().collect();
        let s2: BTreeSet<_> = values.into_iter().collect();
        let domain: Vec<_> = s1.intersection(&s2).copied().collect();

        if domain.is_empty() {
            self.invalidate();
            return Err(VariableError::DomainWipeout);
        }
        let ok_self = {
            let check_change = |var: &mut IntVarValues<T>| {
                if var.size() == domain.len() {
                    IntVariableState::NoChange
                } else if *var.unchecked_min() != unwrap_first!(domain)
                    || *var.unchecked_max() != unwrap_last!(domain)
                {
                    IntVariableState::BoundsChange
                } else {
                    IntVariableState::ValuesChange
                }
            };
            check_change(self)
        };
        self.domain = domain;
        Ok(ok_self)
    }
}

#[cfg(feature = "observer")]
impl<T> OrderedPrunableDomainObserver<T, IntVariableState> for CruspIntVarValues<T>
where
    T: Copy + Clone + Eq + PartialEq + Ord + PartialOrd,
{
    // Change to non-naive implementation
    fn in_sorted_values<Observer, Values>(
        &mut self,
        observer: &mut Observer,
        values: Values,
    ) -> Result<IntVariableState, VariableError>
    where
        Observer: VariableObserver<IntVariableState>,
        Values: IntoIterator<Item = T>,
    {
        use std::collections::BTreeSet;
        let s1: BTreeSet<_> = self.domain.iter().copied().collect();
        let s2: BTreeSet<_> = values.into_iter().collect();
        let domain: Vec<_> = s1.intersection(&s2).copied().collect();

        if domain.is_empty() {
            self.invalidate();
           return observer.push_error(self.id, VariableError::DomainWipeout);
        }
        let ok_self = {
            let mut check_change = |var: &mut CruspIntVarValues<T>, vid: VariableId| {
                if var.size() == domain.len() {
                    Ok(IntVariableState::NoChange)
                } else if *var.unchecked_min() != unwrap_first!(domain)
                    || *var.unchecked_max() != unwrap_last!(domain)
                {
                    observer.push_change(vid, IntVariableState::BoundsChange)
                } else {
                    observer.push_change(vid, IntVariableState::ValuesChange)
                }
            };
            let vid = self.id;
            check_change(self, vid)
        };
        self.domain = domain;
        ok_self
    }
}
