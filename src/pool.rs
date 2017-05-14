use nodrop::NoDrop;
use std::borrow::Borrow;
use std::collections::HashSet;
use std::hash::{self, Hash};
use std::sync::RwLock;

// ++++++++++++++++++++ Constant ++++++++++++++++++++

struct Constant<T>
    where T: ToOwned + ?Sized
{
    value: NoDrop<Box<T::Owned>>,
}

impl<T> Constant<T>
    where T: ToOwned + ?Sized
{
    fn new(value: T::Owned) -> Self { Self { value: NoDrop::new(Box::new(value)) } }

    fn as_static_ref(&self) -> &'static T::Owned {
        unsafe { &*(&**self.value as *const T::Owned) }
    }
}

impl<T> Borrow<T> for Constant<T>
    where T: ToOwned + ?Sized
{
    fn borrow(&self) -> &T { (&**self.value).borrow() }
}

impl<T> Hash for Constant<T>
    where T: ToOwned + ?Sized, T::Owned: Hash
{
    fn hash<H>(&self, state: &mut H)
        where H: hash::Hasher
    {
        self.value.hash(state)
    }
}

impl<T> PartialEq for Constant<T>
    where T: ToOwned + ?Sized, T::Owned: PartialEq
{
    fn eq(&self, other: &Self) -> bool { &**self.value == &**other.value }
}

impl<T> Eq for Constant<T> where T: ToOwned + ?Sized, T::Owned: Eq {}

// ++++++++++++++++++++ Pool ++++++++++++++++++++

pub struct Pool<T>
    where T: ToOwned + ?Sized
{
    consts: RwLock<HashSet<Constant<T>>>,
}

impl<T> Pool<T>
    where T: ToOwned + ?Sized, T::Owned: Hash + Eq, T: Hash + Eq
{
    pub fn new() -> Self { Pool { consts: Default::default() } }

    /// `U` may be `T` or `T::Owned`
    pub fn intern<U>(&self, value: U) -> &'static T::Owned
        where U: Borrow<T>, U: Into<T::Owned>
    {
        let mut consts = self.consts.write().unwrap();
        if let Some(c) = consts.get(value.borrow()) {
            return c.as_static_ref();
        }
        let c = Constant::new(value.into());
        let r = c.as_static_ref();
        consts.insert(c);
        r
    }
}
