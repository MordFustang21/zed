use std::{
    borrow::Cow,
    fmt::{self, Debug},
    sync::Arc,
};

#[derive(PartialEq, Eq)]
pub enum ArcCow<'a, T: ?Sized> {
    Borrowed(&'a T),
    Owned(Arc<T>),
}

use std::hash::{Hash, Hasher};

impl<'a, T: ?Sized + Hash> Hash for ArcCow<'a, T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Borrowed(borrowed) => Hash::hash(borrowed, state),
            Self::Owned(owned) => Hash::hash(&**owned, state),
        }
    }
}

impl<'a, T: ?Sized> Clone for ArcCow<'a, T> {
    fn clone(&self) -> Self {
        match self {
            Self::Borrowed(borrowed) => Self::Borrowed(borrowed),
            Self::Owned(owned) => Self::Owned(owned.clone()),
        }
    }
}

impl<'a, T: ?Sized> From<&'a T> for ArcCow<'a, T> {
    fn from(s: &'a T) -> Self {
        Self::Borrowed(s)
    }
}

impl<T> From<Arc<T>> for ArcCow<'_, T> {
    fn from(s: Arc<T>) -> Self {
        Self::Owned(s)
    }
}

impl From<String> for ArcCow<'_, str> {
    fn from(value: String) -> Self {
        Self::Owned(value.into())
    }
}

impl<'a> From<Cow<'a, str>> for ArcCow<'a, str> {
    fn from(value: Cow<'a, str>) -> Self {
        match value {
            Cow::Borrowed(borrowed) => Self::Borrowed(borrowed),
            Cow::Owned(owned) => Self::Owned(owned.into()),
        }
    }
}

impl<T> From<Vec<T>> for ArcCow<'_, [T]> {
    fn from(vec: Vec<T>) -> Self {
        ArcCow::Owned(Arc::from(vec))
    }
}

impl<'a> From<&'a str> for ArcCow<'a, [u8]> {
    fn from(s: &'a str) -> Self {
        ArcCow::Borrowed(s.as_bytes())
    }
}

impl<'a, T: ?Sized + ToOwned> std::borrow::Borrow<T> for ArcCow<'a, T> {
    fn borrow(&self) -> &T {
        match self {
            ArcCow::Borrowed(borrowed) => borrowed,
            ArcCow::Owned(owned) => owned.as_ref(),
        }
    }
}

impl<T: ?Sized> std::ops::Deref for ArcCow<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            ArcCow::Borrowed(s) => s,
            ArcCow::Owned(s) => s.as_ref(),
        }
    }
}

impl<T: ?Sized> AsRef<T> for ArcCow<'_, T> {
    fn as_ref(&self) -> &T {
        match self {
            ArcCow::Borrowed(borrowed) => borrowed,
            ArcCow::Owned(owned) => owned.as_ref(),
        }
    }
}

impl<'a, T: ?Sized + Debug> Debug for ArcCow<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ArcCow::Borrowed(borrowed) => Debug::fmt(borrowed, f),
            ArcCow::Owned(owned) => Debug::fmt(&**owned, f),
        }
    }
}
