#[cfg(feature = "observer")]
use super::VariableObserver;
use super::{Variable, VariableError, VariableState};
#[cfg(feature = "observer")]
use std::marker::PhantomData;

#[cfg(feature = "observer")]
#[derive(std::default::Default)]
pub struct NoOpObserver<VState>
where
    VState: VariableState,
{
    _state: PhantomData<VState>,
}

#[cfg(feature = "observer")]
impl<VState> NoOpObserver<VState>
where
    VState: VariableState,
{
    pub fn new() -> Self {
        NoOpObserver {
            _state: PhantomData,
        }
    }
}

/// Trait that defines variables with finite domains. In other words the number of elements
/// of the domain is countable). Every variable should have a finite domain.
pub trait FiniteDomain<Type>: Variable<Type> {
    /// The number of elements of the domain.
    fn size(&self) -> usize;
}

/// Trait that definies variable allowing to iter through the elements of its domain.
pub trait IterableDomain<Type>: FiniteDomain<Type> {
    /// Returns an `Iterator` over the elements of the domain.
    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = &Type> + 'a>;
}

/// Trait that defines variable that can be assigned to a specific value.
pub trait AssignableDomain<Type, VState>
where
    VState: VariableState,
{
    /// Change the value of the variable.
    /// Returns an error of type `VariableError::DomainWipeout`
    /// if value is not inside the domain, otherwise returns the correct `VariableState`;
    ///
    /// # Argument
    /// * `value` - The value to assign.
    fn set_value(&mut self, value: Type) -> Result<VState, VariableError>;
}

/// Trait that defines variable that can be assigned to a specific value.
#[cfg(feature = "observer")]
pub trait AssignableDomainObserver<Type, VState>
where
    VState: VariableState,
{
    /// Change the value of the variable.
    /// Returns an error of type `VariableError::DomainWipeout`
    /// if value is not inside the domain, otherwise returns the correct `VariableState`;
    ///
    /// # Argument
    /// * `Observer` - An Observer handler which should call on any change.
    /// * `value` - The value to assign.
    fn set_value<Observer>(
        &mut self,
        observer: &mut Observer,
        value: Type,
    ) -> Result<VState, VariableError>
    where
        Observer: VariableObserver<VState>;
}

/// Trait that defines variable which the underlying `Type` implements the `Ord`
/// trait (i.e. the underlying type is totally ordered).
pub trait OrderedDomain<Type, VState>: FiniteDomain<Type>
where
    VState: VariableState,
    Type: Ord + Eq,
{
    /// Returns the minimal value of the domain.
    fn min(&self) -> Option<&Type>;
    /// Returns the maximal value of the domain.
    fn max(&self) -> Option<&Type>;
    fn unchecked_min(&self) -> &Type {
        let error = format!(
            "Call unchecked_min on a variable with an empty domain (line {}).",
            line!()
        );
        self.min().expect(&error)
    }
    fn unchecked_max(&self) -> &Type {
        let error = format!(
            "Call unchecked_min on a variable with an empty domain (line {}).",
            line!()
        );
        self.max().expect(&error)
    }
    fn strict_upperbound(&mut self, ub: &Type) -> Result<VState, VariableError>;
    fn weak_upperbound(&mut self, ub: &Type) -> Result<VState, VariableError>;
    fn strict_lowerbound(&mut self, lb: &Type) -> Result<VState, VariableError>;
    fn weak_lowerbound(&mut self, lb: &Type) -> Result<VState, VariableError>;
}

/// Trait that defines variable which the underlying `Type` implements the `Ord`
/// trait (i.e. the underlying type is totally ordered).
#[cfg(feature = "observer")]
pub trait OrderedDomainObserver<Type, VState>: FiniteDomain<Type>
where
    VState: VariableState,
    Type: Ord + Eq,
{
    /// Returns the minimal value of the domain.
    fn min(&self) -> Option<&Type>;
    /// Returns the maximal value of the domain.
    fn max(&self) -> Option<&Type>;
    fn unchecked_min(&self) -> &Type {
        let error = format!(
            "Call unchecked_min on a variable with an empty domain (line {}).",
            line!()
        );
        self.min().expect(&error)
    }
    fn unchecked_max(&self) -> &Type {
        let error = format!(
            "Call unchecked_min on a variable with an empty domain (line {}).",
            line!()
        );
        self.max().expect(&error)
    }
    fn strict_upperbound<Observer>(
        &mut self,
        observer: &mut Observer,
        ub: &Type,
    ) -> Result<VState, VariableError>
    where
        Observer: VariableObserver<VState>;
    fn weak_upperbound<Observer>(
        &mut self,
        observer: &mut Observer,
        ub: &Type,
    ) -> Result<VState, VariableError>
    where
        Observer: VariableObserver<VState>;
    fn strict_lowerbound<Observer>(
        &mut self,
        observer: &mut Observer,
        lb: &Type,
    ) -> Result<VState, VariableError>
    where
        Observer: VariableObserver<VState>;
    fn weak_lowerbound<Observer>(
        &mut self,
        observer: &mut Observer,
        lb: &Type,
    ) -> Result<VState, VariableError>
    where
        Observer: VariableObserver<VState>;
}

/// Trait that definies variable that allows to remove any values from its domains.
pub trait EqualDomain<Type, VState, Other = Self>: FiniteDomain<Type>
where
    Type: Eq,
    VState: VariableState,
{
    /// Forces the domain of two variables to be equal.
    ///
    /// # Parameters
    /// * `value` - The variable to compare to.
    fn equal(&mut self, value: &mut Other) -> Result<(VState, VState), VariableError>;
    /// Forces the value of two varaibles to be distinct.
    ///
    /// # Parameters
    /// * `value` - The variable to compare to.
    fn not_equal(&mut self, value: &mut Other) -> Result<(VState, VState), VariableError>;
}

/// Trait that definies variable that allows to remove any values from its domains.
#[cfg(feature = "observer")]
pub trait EqualDomainObserver<Type, VState, Other = Self>: FiniteDomain<Type>
where
    Type: Eq,
    VState: VariableState,
{
    /// Forces the domain of two variables to be equal.
    ///
    /// # Parameters
    /// * `value` - The variable to compare to.
    fn equal<Observer>(
        &mut self,
        observer: &mut Observer,
        value: &mut Other,
    ) -> Result<(VState, VState), VariableError>
    where
        Observer: VariableObserver<VState>;
    /// Forces the value of two varaibles to be distinct.
    ///
    /// # Parameters
    /// * `value` - The variable to compare to.
    fn not_equal<Observer>(
        &mut self,
        observer: &mut Observer,
        value: &mut Other,
    ) -> Result<(VState, VState), VariableError>
    where
        Observer: VariableObserver<VState>;
}

/// Trait that definies variable that allows to remove any values from its domains.
pub trait PrunableDomain<Type, VState>: FiniteDomain<Type>
where
    Type: Eq,
    VState: VariableState,
{
    /// Forces the domain of the variables to be in the values past has parameter.
    ///
    /// # Parameters
    /// * `value` - The variable to compare to.
    fn in_values<Values>(&mut self, values: Values) -> Result<VState, VariableError>
    where
        Values: IntoIterator<Item = Type>;
    /// Remove the value from the domain of a variable.
    ///
    /// # Parameters
    /// * `value` - The variable to compare to.
    fn remove_value(&mut self, value: Type) -> Result<VState, VariableError>;
    /// Remove the values of the domain that satisfies the predicate.
    ///
    /// # Parameters
    /// * `value` - The variable to compare to.
    fn remove_if<Predicate>(&mut self, pred: Predicate) -> Result<VState, VariableError>
    where
        Predicate: FnMut(&Type) -> bool;
    /// Remove the values of the domain that does not satisfy the predicate.
    ///
    /// # Parameters
    /// * `value` - The variable to compare to.
    fn retains_if<Predicate>(&mut self, pred: Predicate) -> Result<VState, VariableError>
    where
        Predicate: FnMut(&Type) -> bool;
}

/// Trait that definies variable that allows to remove any values from its domains.
#[cfg(feature = "observer")]
pub trait PrunableDomainObserver<Type, VState>: FiniteDomain<Type>
where
    Type: Eq,
    VState: VariableState,
{
    /// Forces the domain of the variables to be in the values past has parameter.
    ///
    /// # Parameters
    /// * `value` - The variable to compare to.
    fn in_values<Observer, Values>(
        &mut self,
        observer: &mut Observer,
        values: Values,
    ) -> Result<VState, VariableError>
    where
        Observer: VariableObserver<VState>,
        Values: IntoIterator<Item = Type>;
    /// Remove the value from the domain of a variable.
    ///
    /// # Parameters
    /// * `value` - The variable to compare to.
    fn remove_value<Observer>(
        &mut self,
        observer: &mut Observer,
        value: Type,
    ) -> Result<VState, VariableError>
    where
        Observer: VariableObserver<VState>;
    /// Remove the values of the domain that satisfies the predicate.
    ///
    /// # Parameters
    /// * `value` - The variable to compare to.
    fn remove_if<Observer, Predicate>(
        &mut self,
        observer: &mut Observer,
        pred: Predicate,
    ) -> Result<VState, VariableError>
    where
        Observer: VariableObserver<VState>,
        Predicate: FnMut(&Type) -> bool;
    /// Remove the values of the domain that does not satisfy the predicate.
    ///
    /// # Parameters
    /// * `value` - The variable to compare to.
    fn retains_if<Observer, Predicate>(
        &mut self,
        observer: &mut Observer,
        pred: Predicate,
    ) -> Result<VState, VariableError>
    where
        Observer: VariableObserver<VState>,
        Predicate: FnMut(&Type) -> bool;
}

/// Trait that definies variable that allows to remove any values from its domains.
pub trait OrderedPrunableDomain<Type, VState>:
    EqualDomain<Type, VState> + OrderedDomain<Type, VState>
where
    Type: Eq + Ord,
    VState: VariableState,
{
    /// Forces the domain of the variables to be in the values past has parameter.
    ///
    /// # Parameters
    /// * `value` - The variable to compare to.
    fn in_sorted_values<Values: Iterator<Item = Type>>(
        &mut self,
        values: Values,
    ) -> Result<VState, VariableError>
    where
        Values: IntoIterator<Item = Type>;
}

/// Trait that definies variable that allows to remove any values from its domains.
#[cfg(feature = "observer")]
pub trait OrderedPrunableDomainObserver<Type, VState>:
    EqualDomainObserver<Type, VState> + OrderedDomainObserver<Type, VState>
where
    Type: Eq + Ord,
    VState: VariableState,
{
    /// Forces the domain of the variables to be in the values past has parameter.
    ///
    /// # Parameters
    /// * `value` - The variable to compare to.
    fn in_sorted_values<Observer, Values: Iterator<Item = Type>>(
        &mut self,
        observer: &mut Observer,
        values: Values,
    ) -> Result<VState, VariableError>
    where
        Observer: VariableObserver<VState>,
        Values: IntoIterator<Item = Type>;
}

/// Trait that defines variableswhich the domain can be deduced from an interval.
pub trait FromRangeDomain<Type>: FiniteDomain<Type> {
    /// Returns a new variable from an interval or return `None` if the interval is not valid (max <
    /// min). The domain of the new created variable contains `min` and `max`.
    ///
    /// # Parameters
    /// * `min` - The minimal value of the interval.
    /// * `max` - The maximal value of the interval.
    fn new_from_range(min: Type, max: Type) -> Option<Self>;
}

/// Trait that defines variable which the domain can be deduced from a list of values.
pub trait FromValuesDomain<Type>: FiniteDomain<Type> + Sized {
    /// Returns a new variable from an `Iterator` of values or `None` if the list
    /// of values is empty.
    ///
    /// # Parameters
    /// * `values` - The values of the domain.
    fn new_from_values<Values>(values: Values) -> Option<Self>
    where
        Values: IntoIterator<Item = Type>;
}

#[cfg(feature = "observer")]
pub trait BoundedDomainObserver<Type, VState, Other = Self>:
    OrderedDomainObserver<Type, VState>
where
    VState: VariableState,
    Type: Ord + Eq,
    Other: OrderedDomainObserver<Type, VState>,
{
    /// Forces the domain of `self` to satisfies a precedence relation
    /// with `value`.
    /// Returns an error of type `VariableError::DomainWipeout` if
    /// the minimal value of `self` is greater or equal to the maximal
    /// value of `value`, otherwise returns the correct `VariableState`.
    ///
    /// # Parameters
    /// * `value` - The variable to compare to.
    fn less_than<Observer>(
        &mut self,
        observer: &mut Observer,
        value: &mut Other,
    ) -> Result<(VState, VState), VariableError>
    where
        Observer: VariableObserver<VState>,
    {
        let state_self = self.strict_upperbound(observer, value.unchecked_max().clone())?;
        let state_value = value.strict_lowerbound(observer, self.unchecked_min().clone())?;
        Ok((state_self, state_value))
    }
    /// Forces the domain of `self` to satisfies a weak precedence relation
    /// with `value`.
    /// Returns an error of type `VariableError::DomainWipeout` if
    /// the minimal value of `self` is greater to the maximal
    /// value of `value`, otherwise returns the correct `VariableState`.
    ///
    /// # Parameters
    /// * `value` - The variable to compare to.
    fn less_or_equal_than<Observer>(
        &mut self,
        observer: &mut Observer,
        value: &mut Other,
    ) -> Result<(VState, VState), VariableError>
    where
        Observer: VariableObserver<VState>,
    {
        let state_self = self.weak_upperbound(observer, value.unchecked_max())?;
        let state_value = value.weak_lowerbound(observer, self.unchecked_min())?;
        Ok((state_self, state_value))
    }
    /// Forces the domain of `value` to satisfies a strict precedence relation
    /// with `self`.
    /// Returns an error of type `VariableError::DomainWipeout` if
    /// the minimal value of `value` is greater or equal to the maximal
    /// value of `self`, otherwise returns the correct `VariableState`.
    ///
    /// # Parameters
    /// * `value` - The variable to compare to.
    fn greater_than<Observer>(
        &mut self,
        observer: &mut Observer,
        value: &mut Other,
    ) -> Result<(VState, VState), VariableError>
    where
        Observer: VariableObserver<VState>,
    {
        let state_self = self.strict_lowerbound(observer, value.unchecked_min())?;
        let state_value = value.strict_upperbound(observer, self.unchecked_max())?;
        Ok((state_self, state_value))
    }

    /// Forces the domain of `value` to satisfies a weak precedence relation
    /// with `self`.
    /// Returns an error of type `VariableError::DomainWipeout` if
    /// the minimal value of `value` is greater to the maximal
    /// value of `self`, otherwise returns the correct `VariableState`.
    ///
    /// # Parameters
    /// * `value` - The variable to compare to.
    fn greater_or_equal_than<Observer>(
        &mut self,
        observer: &mut Observer,
        value: &mut Other,
    ) -> Result<(VState, VState), VariableError>
    where
        Observer: VariableObserver<VState>,
    {
        let state_self = self.weak_lowerbound(observer, value.unchecked_min())?;
        let state_value = value.weak_upperbound(observer, self.unchecked_max())?;
        Ok((state_self, state_value))
    }
    /// Forces the domains of two variables two have the same bounds (the does not imply to have
    /// the same domain).
    /// Returns an error of type `VariableError::DomainWipeout` if
    /// the two variables can't have the same bounds (i.e. no common value),
    /// otherwise returns the correct `VariableState`.
    ///
    /// # Parameters
    /// * `value` - The variable to compare to.
    fn equal_bounds_lazy<Observer>(
        &mut self,
        observer: &mut Observer,
        value: &mut Other,
    ) -> Result<(VState, VState), VariableError>
    where
        Observer: VariableObserver<VState>,
    {
        let (x1, y1) = self.less_or_equal_than(observer, value)?;
        let (x2, y2) = self.greater_or_equal_than(observer, value)?;

        Ok((x1 | x2, y1 | y2))
    }

    fn equal_bounds<Observer>(
        &mut self,
        observer: &mut Observer,
        value: &mut Other,
    ) -> Result<(VState, VState), VariableError>
    where
        Observer: VariableObserver<VState>,
    {
        let mut x = VState::null();
        let mut y = VState::null();
        loop {
            let (x1, y1) = self.less_or_equal_than(observer, value)?;
            let (x2, y2) = self.greater_or_equal_than(observer, value)?;
            let new_x = x1 | x2;
            let new_y = y1 | y2;
            if (new_x == VState::null()) && (new_y == VState::null()) {
                break;
            }
            x = x | new_x;
            y = y | new_y;
        }
        Ok((x, y))
    }
}
