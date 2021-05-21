use crusp_core::{Subsumed};
#[cfg(feature = "graph")]
use crusp_core::{VariableId};
use std::marker::PhantomData;

pub mod bool_var;
pub mod domains;
pub mod int_var;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SetVariableState {
    MeSetNone,
    MeSetFailed,
    MeSetVal,
    MeSetCard,
    MeSetLub,
    MeSetGlb,
    MeSetBb,
    MeSetClub,
    MeSetCglb,
    MeSetCbb,
    PcSetVal,
    PcSetCard,
    PcSetClub,
    PcSetCglb,
    PcSetAny,
    PcSetNone,
}

/// Represents an error that occured during variable domain update.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VariableError {
    /// The domain of the variable is empty.
    DomainWipeout,
}
pub trait VariableState: std::ops::BitOr<Output = Self> + Subsumed + Sized {}

#[cfg(feature = "graph")]
pub trait CruspVariable<Type>: Variable<Type> {
    /// Returns the id of the variable
    fn id(&self) -> VariableId;
}

/// Trait for types that represent decision variables.
/// A decision variable is variable along side with its domain of allowed values.
/// A variable has to be cloneable because the (tree based) searching process is based on cloning.
pub trait Variable<Type>: Clone {
    /// Returns if the variable is affected.
    /// A variable is affected if and only if its a domain is a singleton.
    fn is_affected(&self) -> bool;
    /// Returns the value of the variable or `None` if the variable is not
    /// affected.
    fn value(&self) -> Option<&Type>;
}

/// This trait describes an array of variables. There is two types of array:
/// array of variables and array of references to variables. Both types are manipulated with the
/// same trait. When writting constraints over an array of variables, you should use the `Array`
/// trait instead of the specific types `ArrayOfVars` or `ArrayOfRefs`.
pub trait ArrayOfVariables<Type, ArrayVar>
where
    ArrayVar: Variable<Type>,
{
    /// Returns a mutable reference to the variable at that position or None if out of bounds.
    fn get_mut(&mut self, position: usize) -> Option<&mut ArrayVar>;
    /// Returns a reference to the variable at that position or None if out of bounds.
    fn get(&self, position: usize) -> Option<&ArrayVar>;
    /// Returns a mutable reference to the variable at that position without doing bounds check.
    fn get_unchecked_mut(&mut self, position: usize) -> &mut ArrayVar;
    /// Returns a reference to the variable at that position without doing bounds check.
    fn get_unchecked(&self, position: usize) -> &ArrayVar;
    /// Returns an iterator over the variables.
    fn iter<'array>(&'array self) -> Box<dyn Iterator<Item = &ArrayVar> + 'array>;
    /// Returns an iterator that allows modifying each variable.
    fn iter_mut<'array>(&'array mut self) -> Box<dyn Iterator<Item = &mut ArrayVar> + 'array>;
    /// Returns the number of variables.
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0usize
    }
}

/// Represents an array of `Variable`.
#[derive(Debug, Clone)]
pub struct ArrayOfVars<Type, Var>
where
    Var: Variable<Type>,
{
    /// The array of `Variable`.
    variables: Vec<Var>,
    _type: PhantomData<Type>,
}

impl<Type, Var> ArrayOfVars<Type, Var>
where
    Var: Variable<Type>,
{
    /// Creates a new `ArrayOfVars` or None if the number of variables is null.
    ///
    /// # Arguments
    /// *`len` - The number of variables.
    /// *`var` - The prototype of variable used to fill the array.
    pub fn new(len: usize, var: Var) -> Option<Self> {
        Some(ArrayOfVars {
            variables: vec![var; len],
            _type: PhantomData,
        })
    }
    ///
    /// # Arguments
    pub fn new_from_iter(var: impl IntoIterator<Item = Var>) -> Option<Self> {
        Some(ArrayOfVars {
            variables: var.into_iter().collect(),
            _type: PhantomData,
        })
    }
}

impl<Type, Var> ArrayOfVariables<Type, Var> for ArrayOfVars<Type, Var>
where
    Type: Clone,
    Var: Variable<Type>,
{
    fn get_mut(&mut self, position: usize) -> Option<&mut Var> {
        self.variables.get_mut(position)
    }

    fn get(&self, position: usize) -> Option<&Var> {
        self.variables.get(position)
    }

    fn get_unchecked_mut(&mut self, position: usize) -> &mut Var {
        unsafe { &mut *(self.variables.get_unchecked_mut(position) as *mut _) }
    }

    fn get_unchecked(&self, position: usize) -> &Var {
        unsafe { self.variables.get_unchecked(position) }
    }

    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = &Var> + 'a> {
        Box::new(self.variables.iter())
    }

    fn iter_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = &mut Var> + 'a> {
        Box::new(self.variables.iter_mut())
    }

    fn len(&self) -> usize {
        self.variables.len()
    }
}

/// Represents an array of references to `Variable`.
#[derive(Debug, Clone)]
pub struct ArrayOfRefs<Type, Var>
where
    Var: Variable<Type>,
{
    /// The array of references to `Variable`.
    variables: Vec<*mut Var>,
    _type: PhantomData<Type>,
}

impl<Type, Var> ArrayOfRefs<Type, Var>
where
    Type: Clone,
    Var: Variable<Type>,
{
    /// Creates a new `ArrayOfVars` or None if the number of variables is null.
    ///
    /// # Argument
    /// *`variables` - Vector of references to variables.
    pub fn new(variables: Vec<*mut Var>) -> Option<Self> {
        Some(ArrayOfRefs {
            variables,
            _type: PhantomData,
        })
    }
}

impl<Type, Var> ArrayOfVariables<Type, Var> for ArrayOfRefs<Type, Var>
where
    Type: Clone,
    Var: Variable<Type>,
{
    fn get_mut(&mut self, position: usize) -> Option<&mut Var> {
        unsafe { self.variables.get_mut(position).map(|var| &mut (**var)) }
    }

    fn get(&self, position: usize) -> Option<&Var> {
        unsafe { self.variables.get(position).map(|var| &(**var)) }
    }

    fn get_unchecked_mut(&mut self, position: usize) -> &mut Var {
        unsafe { &mut (**self.variables.get_unchecked_mut(position)) }
    }

    fn get_unchecked(&self, position: usize) -> &Var {
        unsafe { &(**self.variables.get_unchecked(position)) }
    }

    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = &Var> + 'a> {
        unsafe { Box::new(self.variables.iter().map(|&var| &*var)) }
    }

    fn iter_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = &mut Var> + 'a> {
        unsafe { Box::new(self.variables.iter_mut().map(|&mut var| &mut *var)) }
    }

    fn len(&self) -> usize {
        self.variables.len()
    }
}
