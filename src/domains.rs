use super::{Variable, VariableError, VariableState};
use crusp_core::VariableId;
use crusp_graph::{GraphEvent, InputEventHandler};
use std::marker::PhantomData;

#[derive(std::default::Default)]
pub struct NoOpEvents<VState>
where
    VState: VariableState + GraphEvent,
{
    _state: PhantomData<VState>,
}

impl<VState> NoOpEvents<VState>
where
    VState: VariableState + GraphEvent,
{
    pub fn new() -> Self {
        NoOpEvents {
            _state: PhantomData,
        }
    }
}

impl<VState> InputEventHandler<VariableId, VState> for NoOpEvents<VState>
where
    VState: VariableState + GraphEvent,
{
    fn notify(&mut self, _node: &VariableId, _event: &VState) -> bool {
        true
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
pub trait AssignableDomainEvents<Type, VState>
where
    VState: VariableState + GraphEvent,
{
    /// Change the value of the variable.
    /// Returns an error of type `VariableError::DomainWipeout`
    /// if value is not inside the domain, otherwise returns the correct `VariableState`;
    ///
    /// # Argument
    /// * `events` - An events handler which should call on any change.
    /// * `value` - The value to assign.
    fn set_value<Events>(
        &mut self,
        events: &mut Events,
        value: Type,
    ) -> Result<VState, VariableError>
    where
        Events: InputEventHandler<VariableId, VState>;
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
    fn strict_upperbound(&mut self, ub: Type) -> Result<VState, VariableError>;
    fn weak_upperbound(&mut self, ub: Type) -> Result<VState, VariableError>;
    fn strict_lowerbound(&mut self, lb: Type) -> Result<VState, VariableError>;
    fn weak_lowerbound(&mut self, lb: Type) -> Result<VState, VariableError>;
}

/// Trait that defines variable which the underlying `Type` implements the `Ord`
/// trait (i.e. the underlying type is totally ordered).
pub trait OrderedDomainEvents<Type, VState>: FiniteDomain<Type>
where
    VState: VariableState + GraphEvent,
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
    fn strict_upperbound<Events>(
        &mut self,
        events: &mut Events,
        ub: Type,
    ) -> Result<VState, VariableError>
    where
        Events: InputEventHandler<VariableId, VState>;
    fn weak_upperbound<Events>(
        &mut self,
        events: &mut Events,
        ub: Type,
    ) -> Result<VState, VariableError>
    where
        Events: InputEventHandler<VariableId, VState>;
    fn strict_lowerbound<Events>(
        &mut self,
        events: &mut Events,
        lb: Type,
    ) -> Result<VState, VariableError>
    where
        Events: InputEventHandler<VariableId, VState>;
    fn weak_lowerbound<Events>(
        &mut self,
        events: &mut Events,
        lb: Type,
    ) -> Result<VState, VariableError>
    where
        Events: InputEventHandler<VariableId, VState>;
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
pub trait EqualDomainEvents<Type, VState, Other = Self>: FiniteDomain<Type>
where
    Type: Eq,
    VState: VariableState + GraphEvent,
{
    /// Forces the domain of two variables to be equal.
    ///
    /// # Parameters
    /// * `value` - The variable to compare to.
    fn equal<Events>(
        &mut self,
        events: &mut Events,
        value: &mut Other,
    ) -> Result<(VState, VState), VariableError>
    where
        Events: InputEventHandler<VariableId, VState>;
    /// Forces the value of two varaibles to be distinct.
    ///
    /// # Parameters
    /// * `value` - The variable to compare to.
    fn not_equal<Events>(
        &mut self,
        events: &mut Events,
        value: &mut Other,
    ) -> Result<(VState, VState), VariableError>
    where
        Events: InputEventHandler<VariableId, VState>;
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
pub trait PrunableDomainEvents<Type, VState>: FiniteDomain<Type>
where
    Type: Eq,
    VState: VariableState + GraphEvent,
{
    /// Forces the domain of the variables to be in the values past has parameter.
    ///
    /// # Parameters
    /// * `value` - The variable to compare to.
    fn in_values<Events, Values>(
        &mut self,
        events: &mut Events,
        values: Values,
    ) -> Result<VState, VariableError>
    where
        Events: InputEventHandler<VariableId, VState>,
        Values: IntoIterator<Item = Type>;
    /// Remove the value from the domain of a variable.
    ///
    /// # Parameters
    /// * `value` - The variable to compare to.
    fn remove_value<Events>(
        &mut self,
        events: &mut Events,
        value: Type,
    ) -> Result<VState, VariableError>
    where
        Events: InputEventHandler<VariableId, VState>;
    /// Remove the values of the domain that satisfies the predicate.
    ///
    /// # Parameters
    /// * `value` - The variable to compare to.
    fn remove_if<Events, Predicate>(
        &mut self,
        events: &mut Events,
        pred: Predicate,
    ) -> Result<VState, VariableError>
    where
        Events: InputEventHandler<VariableId, VState>,
        Predicate: FnMut(&Type) -> bool;
    /// Remove the values of the domain that does not satisfy the predicate.
    ///
    /// # Parameters
    /// * `value` - The variable to compare to.
    fn retains_if<Events, Predicate>(
        &mut self,
        events: &mut Events,
        pred: Predicate,
    ) -> Result<VState, VariableError>
    where
        Events: InputEventHandler<VariableId, VState>,
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
pub trait OrderedPrunableDomainEvents<Type, VState>:
    EqualDomainEvents<Type, VState> + OrderedDomainEvents<Type, VState>
where
    Type: Eq + Ord,
    VState: VariableState + GraphEvent,
{
    /// Forces the domain of the variables to be in the values past has parameter.
    ///
    /// # Parameters
    /// * `value` - The variable to compare to.
    fn in_sorted_values<Events, Values: Iterator<Item = Type>>(
        &mut self,
        events: &mut Events,
        values: Values,
    ) -> Result<VState, VariableError>
    where
        Events: InputEventHandler<VariableId, VState>,
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
