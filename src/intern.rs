use pool::Pool;
use std::ffi::{CStr, CString, OsStr, OsString};
use std::path::{Path, PathBuf};

pub trait Intern {
    type Interned: 'static;
    fn intern(self) -> &'static Self::Interned;
}

lazy_static!{
    static ref STR_POOL: Pool<str> = Pool::new();
    static ref C_STR_POOL: Pool<CStr> = Pool::new();
    static ref OS_STR_POOL: Pool<OsStr> = Pool::new();
    static ref PATH_POOL: Pool<Path> = Pool::new();
}

impl<'a> Intern for &'a str {
    type Interned = String;
    fn intern(self) -> &'static Self::Interned { STR_POOL.intern(self) }
}
impl Intern for String {
    type Interned = Self;
    fn intern(self) -> &'static Self::Interned { STR_POOL.intern(self) }
}
impl<'a> Intern for &'a String {
    type Interned = String;
    fn intern(self) -> &'static Self::Interned { STR_POOL.intern(&**self) }
}
impl<'a> Intern for &'a mut String {
    type Interned = String;
    fn intern(self) -> &'static Self::Interned { STR_POOL.intern(&**self) }
}

impl<'a> Intern for &'a CStr {
    type Interned = CString;
    fn intern(self) -> &'static Self::Interned { C_STR_POOL.intern(self) }
}
impl Intern for CString {
    type Interned = Self;
    fn intern(self) -> &'static Self::Interned { C_STR_POOL.intern(self) }
}
impl<'a> Intern for &'a CString {
    type Interned = CString;
    fn intern(self) -> &'static Self::Interned { C_STR_POOL.intern(&**self) }
}
impl<'a> Intern for &'a mut CString {
    type Interned = CString;
    fn intern(self) -> &'static Self::Interned { C_STR_POOL.intern(&**self) }
}

impl<'a> Intern for &'a OsStr {
    type Interned = OsString;
    fn intern(self) -> &'static Self::Interned { OS_STR_POOL.intern(self) }
}
impl Intern for OsString {
    type Interned = Self;
    fn intern(self) -> &'static Self::Interned { OS_STR_POOL.intern(self) }
}
impl<'a> Intern for &'a OsString {
    type Interned = OsString;
    fn intern(self) -> &'static Self::Interned { OS_STR_POOL.intern(&**self) }
}
impl<'a> Intern for &'a mut OsString {
    type Interned = OsString;
    fn intern(self) -> &'static Self::Interned { OS_STR_POOL.intern(&**self) }
}

impl<'a> Intern for &'a Path {
    type Interned = PathBuf;
    fn intern(self) -> &'static Self::Interned { PATH_POOL.intern(self) }
}
impl Intern for PathBuf {
    type Interned = Self;
    fn intern(self) -> &'static Self::Interned { PATH_POOL.intern(self) }
}
impl<'a> Intern for &'a PathBuf {
    type Interned = PathBuf;
    fn intern(self) -> &'static Self::Interned { PATH_POOL.intern(&**self) }
}
impl<'a> Intern for &'a mut PathBuf {
    type Interned = PathBuf;
    fn intern(self) -> &'static Self::Interned { PATH_POOL.intern(&**self) }
}
