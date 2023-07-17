#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::any::{Any, TypeId};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::hash::Hash;

pub trait ConvReprC {
    type ReprC;
    fn into_repr_c(self) -> Self::ReprC;
    fn from_repr_c(c: Self::ReprC) -> Self;
}

impl<A> ConvReprC for (A, ) where A: ConvReprC {
    type ReprC = (A::ReprC, );
    #[inline]
    fn into_repr_c(self) -> Self::ReprC {
        let (a, ) = self;
        (a.into_repr_c(), )
    }
    #[inline]
    fn from_repr_c((a, ): Self::ReprC) -> Self {
        (A::from_repr_c(a), )
    }
}

impl<A, B> ConvReprC for (A, B) where
    A: ConvReprC,
    B: ConvReprC,
{
    type ReprC = (A::ReprC, B::ReprC);
    #[inline]
    fn into_repr_c(self) -> Self::ReprC {
        let (a, b) = self;
        (a.into_repr_c(), b.into_repr_c())
    }
    #[inline]
    fn from_repr_c((a, b): Self::ReprC) -> Self {
        (A::from_repr_c(a), B::from_repr_c(b))
    }
}

impl<A, B, C> ConvReprC for (A, B, C) where
    A: ConvReprC,
    B: ConvReprC,
    C: ConvReprC,
{
    type ReprC = (A::ReprC, B::ReprC, C::ReprC);
    #[inline]
    fn into_repr_c(self) -> Self::ReprC {
        let (a, b, c) = self;
        (a.into_repr_c(), b.into_repr_c(), c.into_repr_c())
    }
    #[inline]
    fn from_repr_c((a, b, c): Self::ReprC) -> Self {
        (A::from_repr_c(a), B::from_repr_c(b), C::from_repr_c(c))
    }
}


impl<A, B, C, D> ConvReprC for (A, B, C, D) where
    A: ConvReprC,
    B: ConvReprC,
    C: ConvReprC,
    D: ConvReprC,
{
    type ReprC = (A::ReprC, B::ReprC, C::ReprC, D::ReprC);
    #[inline]
    fn into_repr_c(self) -> Self::ReprC {
        let (a, b, c, d) = self;
        (a.into_repr_c(), b.into_repr_c(), c.into_repr_c(), d.into_repr_c())
    }
    #[inline]
    fn from_repr_c((a, b, c, d): Self::ReprC) -> Self {
        (A::from_repr_c(a), B::from_repr_c(b), C::from_repr_c(c), D::from_repr_c(d))
    }
}


impl<A, B, C, D, E> ConvReprC for (A, B, C, D, E) where
    A: ConvReprC,
    B: ConvReprC,
    C: ConvReprC,
    D: ConvReprC,
    E: ConvReprC,
{
    type ReprC = (A::ReprC, B::ReprC, C::ReprC, D::ReprC, E::ReprC);
    #[inline]
    fn into_repr_c(self) -> Self::ReprC {
        let (a, b, c, d, e) = self;
        (a.into_repr_c(), b.into_repr_c(), c.into_repr_c(), d.into_repr_c(), e.into_repr_c())
    }
    #[inline]
    fn from_repr_c((a, b, c, d, e): Self::ReprC) -> Self {
        (A::from_repr_c(a), B::from_repr_c(b), C::from_repr_c(c), D::from_repr_c(d), E::from_repr_c(e))
    }
}


impl<A, B, C, D, E, F> ConvReprC for (A, B, C, D, E, F) where
    A: ConvReprC,
    B: ConvReprC,
    C: ConvReprC,
    D: ConvReprC,
    E: ConvReprC,
    F: ConvReprC,
{
    type ReprC = (A::ReprC, B::ReprC, C::ReprC, D::ReprC, E::ReprC, F::ReprC);
    #[inline]
    fn into_repr_c(self) -> Self::ReprC {
        let (a, b, c, d, e, f) = self;
        (a.into_repr_c(), b.into_repr_c(), c.into_repr_c(), d.into_repr_c(), e.into_repr_c(), f.into_repr_c())
    }
    #[inline]
    fn from_repr_c((a, b, c, d, e, f): Self::ReprC) -> Self {
        (A::from_repr_c(a), B::from_repr_c(b), C::from_repr_c(c), D::from_repr_c(d), E::from_repr_c(e), F::from_repr_c(f))
    }
}

impl<A, B, C, D, E, F, G> ConvReprC for (A, B, C, D, E, F, G) where
    A: ConvReprC,
    B: ConvReprC,
    C: ConvReprC,
    D: ConvReprC,
    E: ConvReprC,
    F: ConvReprC,
    G: ConvReprC,
{
    type ReprC = (A::ReprC, B::ReprC, C::ReprC, D::ReprC, E::ReprC, F::ReprC, G::ReprC);
    #[inline]
    fn into_repr_c(self) -> Self::ReprC {
        let (a, b, c, d, e, f, g) = self;
        (a.into_repr_c(), b.into_repr_c(), c.into_repr_c(), d.into_repr_c(), e.into_repr_c(), f.into_repr_c(), g.into_repr_c())
    }
    #[inline]
    fn from_repr_c((a, b, c, d, e, f, g): Self::ReprC) -> Self {
        (A::from_repr_c(a), B::from_repr_c(b), C::from_repr_c(c), D::from_repr_c(d), E::from_repr_c(e), F::from_repr_c(f), G::from_repr_c(g))
    }
}


impl<A, B, C, D, E, F, G, H> ConvReprC for (A, B, C, D, E, F, G, H) where
    A: ConvReprC,
    B: ConvReprC,
    C: ConvReprC,
    D: ConvReprC,
    E: ConvReprC,
    F: ConvReprC,
    G: ConvReprC,
    H: ConvReprC,
{
    type ReprC = (A::ReprC, B::ReprC, C::ReprC, D::ReprC, E::ReprC, F::ReprC, G::ReprC, H::ReprC);
    #[inline]
    fn into_repr_c(self) -> Self::ReprC {
        let (a, b, c, d, e, f, g, h) = self;
        (a.into_repr_c(), b.into_repr_c(), c.into_repr_c(), d.into_repr_c(), e.into_repr_c(), f.into_repr_c(), g.into_repr_c(), h.into_repr_c())
    }
    #[inline]
    fn from_repr_c((a, b, c, d, e, f, g, h): Self::ReprC) -> Self {
        (A::from_repr_c(a), B::from_repr_c(b), C::from_repr_c(c), D::from_repr_c(d), E::from_repr_c(e), F::from_repr_c(f), G::from_repr_c(g), H::from_repr_c(h))
    }
}


impl<A, B, C, D, E, F, G, H, I> ConvReprC for (A, B, C, D, E, F, G, H, I) where
    A: ConvReprC,
    B: ConvReprC,
    C: ConvReprC,
    D: ConvReprC,
    E: ConvReprC,
    F: ConvReprC,
    G: ConvReprC,
    H: ConvReprC,
    I: ConvReprC,
{
    type ReprC = (A::ReprC, B::ReprC, C::ReprC, D::ReprC, E::ReprC, F::ReprC, G::ReprC, H::ReprC, I::ReprC);
    #[inline]
    fn into_repr_c(self) -> Self::ReprC {
        let (a, b, c, d, e, f, g, h, i) = self;
        (a.into_repr_c(), b.into_repr_c(), c.into_repr_c(), d.into_repr_c(), e.into_repr_c(), f.into_repr_c(), g.into_repr_c(), h.into_repr_c(), i.into_repr_c())
    }
    #[inline]
    fn from_repr_c((a, b, c, d, e, f, g, h, i): Self::ReprC) -> Self {
        (A::from_repr_c(a), B::from_repr_c(b), C::from_repr_c(c), D::from_repr_c(d), E::from_repr_c(e), F::from_repr_c(f), G::from_repr_c(g), H::from_repr_c(h), I::from_repr_c(i))
    }
}

impl<A, B, C, D, E, F, G, H, I, J> ConvReprC for (A, B, C, D, E, F, G, H, I, J) where
    A: ConvReprC,
    B: ConvReprC,
    C: ConvReprC,
    D: ConvReprC,
    E: ConvReprC,
    F: ConvReprC,
    G: ConvReprC,
    H: ConvReprC,
    I: ConvReprC,
    J: ConvReprC,
{
    type ReprC = (A::ReprC, B::ReprC, C::ReprC, D::ReprC, E::ReprC, F::ReprC, G::ReprC, H::ReprC, I::ReprC, J::ReprC);
    #[inline]
    fn into_repr_c(self) -> Self::ReprC {
        let (a, b, c, d, e, f, g, h, i, j) = self;
        (a.into_repr_c(), b.into_repr_c(), c.into_repr_c(), d.into_repr_c(), e.into_repr_c(), f.into_repr_c(), g.into_repr_c(), h.into_repr_c(), i.into_repr_c(), j.into_repr_c())
    }
    #[inline]
    fn from_repr_c((a, b, c, d, e, f, g, h, i, j): Self::ReprC) -> Self {
        (A::from_repr_c(a), B::from_repr_c(b), C::from_repr_c(c), D::from_repr_c(d), E::from_repr_c(e), F::from_repr_c(f), G::from_repr_c(g), H::from_repr_c(h), I::from_repr_c(i), J::from_repr_c(j))
    }
}


impl<ReprRust> ConvReprC for Box<ReprRust> where ReprRust: ConvReprC {
    type ReprC = *mut ReprRust::ReprC;

    #[inline]
    fn into_repr_c(self) -> Self::ReprC {
        Box::into_raw(Box::new((*self).into_repr_c()))
    }
    #[inline]
    fn from_repr_c(c: Self::ReprC) -> Self {
        if c.is_null() {
            unsafe { Box::from_raw(std::ptr::null_mut()) }
        } else {
            Box::new(ReprRust::from_repr_c(unsafe { *Box::from_raw(c) }))
        }
    }
}

impl<ReprRust> ConvReprC for Option<Box<ReprRust>> where ReprRust: ConvReprC {
    type ReprC = *mut ReprRust::ReprC;

    #[inline]
    fn into_repr_c(self) -> Self::ReprC {
        if let Some(x) = self {
            x.into_repr_c()
        } else {
            ::std::ptr::null_mut()
        }
    }

    #[inline]
    fn from_repr_c(c: Self::ReprC) -> Self {
        if c.is_null() {
            None
        } else {
            Some(Box::new(ReprRust::from_repr_c(unsafe { *Box::from_raw(c) })))
        }
    }
}

impl ConvReprC for String {
    type ReprC = C_String;

    #[inline]
    fn into_repr_c(self) -> Self::ReprC {
        C_String::from_string(self)
    }

    #[inline]
    fn from_repr_c(c: Self::ReprC) -> Self {
        c.into_string()
    }
}


impl<ReprRust> ConvReprC for Vec<ReprRust> where ReprRust: ConvReprC + Any {
    type ReprC = C_DynArray<ReprRust::ReprC>;

    #[inline]
    fn into_repr_c(self) -> Self::ReprC {
        if self.is_empty() {
            return Self::ReprC::null();
        }
        if TypeId::of::<ReprRust>() == TypeId::of::<ReprRust::ReprC>() {
            C_DynArray {
                len: self.len(),
                cap: self.capacity(),
                ptr: self.leak().as_mut_ptr() as *mut ReprRust::ReprC,
            }
        } else {
            Self::ReprC::from_vec(self.into_iter()
                .map(|v| v.into_repr_c())
                .collect::<Vec<ReprRust::ReprC>>()
            )
        }
    }
    #[inline]
    fn from_repr_c(c: Self::ReprC) -> Self {
        if c.is_empty() {
            return Vec::new();
        }
        if TypeId::of::<ReprRust>() == TypeId::of::<ReprRust::ReprC>() {
            let mut v = unsafe { Vec::from_raw_parts(c.ptr as *mut ReprRust, c.len, c.cap) };
            v.shrink_to_fit();
            v
        } else {
            c.into_vec().into_iter().map(|v| ReprRust::from_repr_c(v)).collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use core::any::TypeId;
    use std::any::Any;

    use crate::ctypes::ConvReprC;

    #[test]
    fn conv_repr_csame_vec() {
        let b = vec![5i64].into_repr_c();
        assert_eq!(vec![5i64], <Vec<i64> as ConvReprC>::from_repr_c(b))
    }

    fn gty<A: Any, B: Any>() -> bool {
        return TypeId::of::<A>() == TypeId::of::<B>();
    }

    #[test]
    fn test_same_gty() {
        type S = String;
        assert!(gty::<i8, i8>());
        assert!(!gty::<i8, i32>());
        assert!(!gty::<i8, String>());
        assert!(gty::<S, String>());
    }
}


impl<ReprRustK, ReprRustV> ConvReprC for Map<ReprRustK, ReprRustV>
    where
        ReprRustK: ConvReprC,
        ReprRustV: ConvReprC,
{
    type ReprC = C_Map<ReprRustK::ReprC, ReprRustV::ReprC>;

    #[inline]
    fn into_repr_c(self) -> Self::ReprC {
        if self.0.is_empty() {
            return Self::ReprC::null();
        }
        Self::ReprC::from_vec(self.0.into_iter()
            .map(|kv| MapEntry { key: kv.key.into_repr_c(), value: kv.value.into_repr_c() })
            .collect::<Vec<MapEntry<ReprRustK::ReprC, ReprRustV::ReprC>>>()
        )
    }
    #[inline]
    fn from_repr_c(c: Self::ReprC) -> Self {
        if c.is_empty() {
            return Map(Vec::new());
        }
        Map(c.into_vec().into_iter().map(|kv| MapEntry { key: ReprRustK::from_repr_c(kv.key), value: ReprRustV::from_repr_c(kv.value) }).collect())
    }
}

impl<ReprRustK, ReprRustV> ConvReprC for HashMap<ReprRustK, ReprRustV>
    where
        ReprRustK: ConvReprC + PartialEq + Eq + Hash,
        ReprRustV: ConvReprC,
{
    type ReprC = C_Map<ReprRustK::ReprC, ReprRustV::ReprC>;

    #[inline]
    fn into_repr_c(self) -> Self::ReprC {
        if self.is_empty() {
            return Self::ReprC::null();
        }
        Self::ReprC::from_vec(self.into_iter()
            .map(|(k, v)| MapEntry { key: k.into_repr_c(), value: v.into_repr_c() })
            .collect::<Vec<MapEntry<ReprRustK::ReprC, ReprRustV::ReprC>>>()
        )
    }
    #[inline]
    fn from_repr_c(c: Self::ReprC) -> Self {
        if c.is_empty() {
            return Self::new();
        }
        Map::<ReprRustK, ReprRustV>(c.into_vec()
            .into_iter()
            .map(|kv| MapEntry { key: ReprRustK::from_repr_c(kv.key), value: ReprRustV::from_repr_c(kv.value) })
            .collect::<Vec<MapEntry<ReprRustK, ReprRustV>>>()
        )
            .into_hash_map()
    }
}


impl<ReprRust> ConvReprC for HashSet<ReprRust>
    where
        ReprRust: ConvReprC + Eq + Hash,
{
    type ReprC = C_Set<ReprRust::ReprC>;

    #[inline]
    fn into_repr_c(self) -> Self::ReprC {
        if self.is_empty() {
            return Self::ReprC::null();
        }
        Self::ReprC::from_vec(self.into_iter()
            .map(|v| v.into_repr_c())
            .collect::<Vec<ReprRust::ReprC>>()
        )
    }
    #[inline]
    fn from_repr_c(c: Self::ReprC) -> Self {
        if c.is_empty() {
            return Self::new();
        }
        HashSet::<ReprRust>::from_iter(c.into_vec()
            .into_iter()
            .map(|v| ReprRust::from_repr_c(v)))
    }
}

macro_rules! impl_scalar_conv_repr_c {
    () => {};
    ($ty:ty; $($tail:tt)*) => {
        impl ConvReprC for $ty {
            type ReprC = $ty;
            #[inline]
            fn into_repr_c(self) -> Self::ReprC {
                self
            }
            #[inline]
            fn from_repr_c(c: Self::ReprC) -> Self {
                c
            }
        }
        impl_scalar_conv_repr_c!($($tail)*);
    }
}

impl_scalar_conv_repr_c!((); bool; i8; i16; i32; i64; i128; u8; u16; u32; u64; u128; f32; f64;);


#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct C_DynArray<T> {
    pub ptr: *mut T,
    pub len: usize,
    pub cap: usize,
}

impl<T> C_DynArray<T> {
    pub fn null() -> Self {
        Self {
            ptr: std::ptr::null_mut(),
            len: 0,
            cap: 0,
        }
    }
    pub fn from_vec(v: Vec<T>) -> Self {
        let s = C_DynArray {
            len: v.len(),
            cap: v.capacity(),
            ptr: v.leak().as_mut_ptr(),
        };
        s
    }
    pub fn into_vec(self) -> Vec<T> {
        if self.is_empty() {
            return Vec::new();
        }
        let mut v = unsafe { Vec::from_raw_parts(self.ptr as *mut T, self.len, self.cap) };
        v.shrink_to_fit();
        v
    }
    pub fn is_empty(&self) -> bool {
        self.ptr.is_null() || self.len == 0 || self.cap == 0
    }
}

type C_Bytes = C_DynArray<u8>;

impl C_DynArray<u8> {
    #[inline]
    pub fn from_bytes(v: Vec<u8>) -> Self {
        Self::from_vec(v)
    }
    #[inline]
    pub fn into_bytes(self) -> Vec<u8> {
        self.into_vec()
    }
}

pub type C_String = C_DynArray<u8>;

impl C_DynArray<u8> {
    #[inline]
    pub fn from_string(s: String) -> Self {
        Self::from_vec(s.into_bytes())
    }
    #[inline]
    pub fn into_string(self) -> String {
        unsafe { String::from_utf8_unchecked(self.into_vec()) }
    }
}

pub type C_Map<K, V> = C_DynArray<MapEntry<K, V>>;

impl<K: Eq + Hash + Ord, V> C_DynArray<MapEntry<K, V>> {
    #[inline]
    pub fn from_map(m: Map<K, V>) -> Self {
        Self::from_vec(m.0)
    }
    #[inline]
    pub fn into_map(self) -> Map<K, V> {
        Map(self.into_vec())
    }
    #[inline]
    pub fn from_hash_map(m: HashMap<K, V>) -> Self {
        Self::from_map(Map::from_hash_map(m))
    }
    #[inline]
    pub fn into_hash_map(self) -> HashMap<K, V> {
        self.into_map().into_hash_map()
    }
    #[inline]
    pub fn from_b_tree_map(m: BTreeMap<K, V>) -> Self {
        Self::from_map(Map::from_b_tree_map(m))
    }
    #[inline]
    pub fn into_b_tree_map(self) -> BTreeMap<K, V> {
        self.into_map().into_b_tree_map()
    }
}

#[repr(C)]
pub struct MapEntry<K, V> {
    pub key: K,
    pub value: V,
}

pub struct Map<K, V>(pub Vec<MapEntry<K, V>>);

impl<K: PartialEq + Eq + Hash, V> Map<K, V> {
    pub fn from_hash_map(m: HashMap<K, V>) -> Self {
        Map(m.into_iter().map(|(key, value)| MapEntry { key, value }).collect())
    }
    pub fn into_hash_map(self) -> HashMap<K, V> {
        let mut m = HashMap::with_capacity(self.0.capacity());
        for x in self.0 {
            m.insert(x.key, x.value);
        }
        m
    }
}


impl<K: Ord, V> Map<K, V> {
    pub fn from_b_tree_map(m: BTreeMap<K, V>) -> Self {
        Map(m.into_iter().map(|(key, value)| MapEntry { key, value }).collect())
    }
    pub fn into_b_tree_map(self) -> BTreeMap<K, V> {
        let mut m = BTreeMap::new();
        for x in self.0 {
            m.insert(x.key, x.value);
        }
        m
    }
}

type C_Set<T> = C_DynArray<T>;

impl<T: Eq + Hash> C_DynArray<T> {
    #[inline]
    pub fn from_hash_set(s: HashSet<T>) -> Self {
        Self::from_vec(s.into_iter().collect())
    }
    #[inline]
    fn into_hash_set(self) -> HashSet<T> {
        HashSet::from_iter(self.into_vec().into_iter())
    }
}


pub struct GoFfiResult<RET, ARGS: ConvReprC> {
    ret: RET,
    c_args: *mut ARGS::ReprC,
    c_ret_ptr: usize,
    c_free_fn: unsafe extern "C" fn(usize),
}

impl<RET, ARGS: ConvReprC> GoFfiResult<RET, ARGS> {
    pub fn new(ret: RET, c_args: *mut ARGS::ReprC, c_ret_ptr: usize, c_free_fn: unsafe extern "C" fn(usize)) -> Self {
        Self {
            ret,
            c_args,
            c_ret_ptr,
            c_free_fn,
        }
    }
    pub fn ret(&self) -> &RET {
        &self.ret
    }
}

impl<RET, ARGS: ConvReprC> Drop for GoFfiResult<RET, ARGS> {
    fn drop(&mut self) {
        let Self { c_args, c_ret_ptr, c_free_fn, .. } = self;
        let _ = ARGS::from_repr_c(*unsafe { Box::from_raw(c_args.clone()) });
        unsafe { c_free_fn(c_ret_ptr.clone()) };
    }
}
