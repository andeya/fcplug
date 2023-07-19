#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::any::{Any, TypeId};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::hash::Hash;

pub trait ConvRepr {
    type CRepr;
    fn into_c_repr(self) -> Self::CRepr;
    fn from_c_repr(c: Self::CRepr) -> Self;
}

impl<A> ConvRepr for (A, ) where A: ConvRepr {
    type CRepr = (A::CRepr, );
    #[inline]
    fn into_c_repr(self) -> Self::CRepr {
        let (a, ) = self;
        (a.into_c_repr(), )
    }
    #[inline]
    fn from_c_repr((a, ): Self::CRepr) -> Self {
        (A::from_c_repr(a), )
    }
}

impl<A, B> ConvRepr for (A, B) where
    A: ConvRepr,
    B: ConvRepr,
{
    type CRepr = (A::CRepr, B::CRepr);
    #[inline]
    fn into_c_repr(self) -> Self::CRepr {
        let (a, b) = self;
        (a.into_c_repr(), b.into_c_repr())
    }
    #[inline]
    fn from_c_repr((a, b): Self::CRepr) -> Self {
        (A::from_c_repr(a), B::from_c_repr(b))
    }
}

impl<A, B, C> ConvRepr for (A, B, C) where
    A: ConvRepr,
    B: ConvRepr,
    C: ConvRepr,
{
    type CRepr = (A::CRepr, B::CRepr, C::CRepr);
    #[inline]
    fn into_c_repr(self) -> Self::CRepr {
        let (a, b, c) = self;
        (a.into_c_repr(), b.into_c_repr(), c.into_c_repr())
    }
    #[inline]
    fn from_c_repr((a, b, c): Self::CRepr) -> Self {
        (A::from_c_repr(a), B::from_c_repr(b), C::from_c_repr(c))
    }
}


impl<A, B, C, D> ConvRepr for (A, B, C, D) where
    A: ConvRepr,
    B: ConvRepr,
    C: ConvRepr,
    D: ConvRepr,
{
    type CRepr = (A::CRepr, B::CRepr, C::CRepr, D::CRepr);
    #[inline]
    fn into_c_repr(self) -> Self::CRepr {
        let (a, b, c, d) = self;
        (a.into_c_repr(), b.into_c_repr(), c.into_c_repr(), d.into_c_repr())
    }
    #[inline]
    fn from_c_repr((a, b, c, d): Self::CRepr) -> Self {
        (A::from_c_repr(a), B::from_c_repr(b), C::from_c_repr(c), D::from_c_repr(d))
    }
}


impl<A, B, C, D, E> ConvRepr for (A, B, C, D, E) where
    A: ConvRepr,
    B: ConvRepr,
    C: ConvRepr,
    D: ConvRepr,
    E: ConvRepr,
{
    type CRepr = (A::CRepr, B::CRepr, C::CRepr, D::CRepr, E::CRepr);
    #[inline]
    fn into_c_repr(self) -> Self::CRepr {
        let (a, b, c, d, e) = self;
        (a.into_c_repr(), b.into_c_repr(), c.into_c_repr(), d.into_c_repr(), e.into_c_repr())
    }
    #[inline]
    fn from_c_repr((a, b, c, d, e): Self::CRepr) -> Self {
        (A::from_c_repr(a), B::from_c_repr(b), C::from_c_repr(c), D::from_c_repr(d), E::from_c_repr(e))
    }
}


impl<A, B, C, D, E, F> ConvRepr for (A, B, C, D, E, F) where
    A: ConvRepr,
    B: ConvRepr,
    C: ConvRepr,
    D: ConvRepr,
    E: ConvRepr,
    F: ConvRepr,
{
    type CRepr = (A::CRepr, B::CRepr, C::CRepr, D::CRepr, E::CRepr, F::CRepr);
    #[inline]
    fn into_c_repr(self) -> Self::CRepr {
        let (a, b, c, d, e, f) = self;
        (a.into_c_repr(), b.into_c_repr(), c.into_c_repr(), d.into_c_repr(), e.into_c_repr(), f.into_c_repr())
    }
    #[inline]
    fn from_c_repr((a, b, c, d, e, f): Self::CRepr) -> Self {
        (A::from_c_repr(a), B::from_c_repr(b), C::from_c_repr(c), D::from_c_repr(d), E::from_c_repr(e), F::from_c_repr(f))
    }
}

impl<A, B, C, D, E, F, G> ConvRepr for (A, B, C, D, E, F, G) where
    A: ConvRepr,
    B: ConvRepr,
    C: ConvRepr,
    D: ConvRepr,
    E: ConvRepr,
    F: ConvRepr,
    G: ConvRepr,
{
    type CRepr = (A::CRepr, B::CRepr, C::CRepr, D::CRepr, E::CRepr, F::CRepr, G::CRepr);
    #[inline]
    fn into_c_repr(self) -> Self::CRepr {
        let (a, b, c, d, e, f, g) = self;
        (a.into_c_repr(), b.into_c_repr(), c.into_c_repr(), d.into_c_repr(), e.into_c_repr(), f.into_c_repr(), g.into_c_repr())
    }
    #[inline]
    fn from_c_repr((a, b, c, d, e, f, g): Self::CRepr) -> Self {
        (A::from_c_repr(a), B::from_c_repr(b), C::from_c_repr(c), D::from_c_repr(d), E::from_c_repr(e), F::from_c_repr(f), G::from_c_repr(g))
    }
}


impl<A, B, C, D, E, F, G, H> ConvRepr for (A, B, C, D, E, F, G, H) where
    A: ConvRepr,
    B: ConvRepr,
    C: ConvRepr,
    D: ConvRepr,
    E: ConvRepr,
    F: ConvRepr,
    G: ConvRepr,
    H: ConvRepr,
{
    type CRepr = (A::CRepr, B::CRepr, C::CRepr, D::CRepr, E::CRepr, F::CRepr, G::CRepr, H::CRepr);
    #[inline]
    fn into_c_repr(self) -> Self::CRepr {
        let (a, b, c, d, e, f, g, h) = self;
        (a.into_c_repr(), b.into_c_repr(), c.into_c_repr(), d.into_c_repr(), e.into_c_repr(), f.into_c_repr(), g.into_c_repr(), h.into_c_repr())
    }
    #[inline]
    fn from_c_repr((a, b, c, d, e, f, g, h): Self::CRepr) -> Self {
        (A::from_c_repr(a), B::from_c_repr(b), C::from_c_repr(c), D::from_c_repr(d), E::from_c_repr(e), F::from_c_repr(f), G::from_c_repr(g), H::from_c_repr(h))
    }
}


impl<A, B, C, D, E, F, G, H, I> ConvRepr for (A, B, C, D, E, F, G, H, I) where
    A: ConvRepr,
    B: ConvRepr,
    C: ConvRepr,
    D: ConvRepr,
    E: ConvRepr,
    F: ConvRepr,
    G: ConvRepr,
    H: ConvRepr,
    I: ConvRepr,
{
    type CRepr = (A::CRepr, B::CRepr, C::CRepr, D::CRepr, E::CRepr, F::CRepr, G::CRepr, H::CRepr, I::CRepr);
    #[inline]
    fn into_c_repr(self) -> Self::CRepr {
        let (a, b, c, d, e, f, g, h, i) = self;
        (a.into_c_repr(), b.into_c_repr(), c.into_c_repr(), d.into_c_repr(), e.into_c_repr(), f.into_c_repr(), g.into_c_repr(), h.into_c_repr(), i.into_c_repr())
    }
    #[inline]
    fn from_c_repr((a, b, c, d, e, f, g, h, i): Self::CRepr) -> Self {
        (A::from_c_repr(a), B::from_c_repr(b), C::from_c_repr(c), D::from_c_repr(d), E::from_c_repr(e), F::from_c_repr(f), G::from_c_repr(g), H::from_c_repr(h), I::from_c_repr(i))
    }
}

impl<A, B, C, D, E, F, G, H, I, J> ConvRepr for (A, B, C, D, E, F, G, H, I, J) where
    A: ConvRepr,
    B: ConvRepr,
    C: ConvRepr,
    D: ConvRepr,
    E: ConvRepr,
    F: ConvRepr,
    G: ConvRepr,
    H: ConvRepr,
    I: ConvRepr,
    J: ConvRepr,
{
    type CRepr = (A::CRepr, B::CRepr, C::CRepr, D::CRepr, E::CRepr, F::CRepr, G::CRepr, H::CRepr, I::CRepr, J::CRepr);
    #[inline]
    fn into_c_repr(self) -> Self::CRepr {
        let (a, b, c, d, e, f, g, h, i, j) = self;
        (a.into_c_repr(), b.into_c_repr(), c.into_c_repr(), d.into_c_repr(), e.into_c_repr(), f.into_c_repr(), g.into_c_repr(), h.into_c_repr(), i.into_c_repr(), j.into_c_repr())
    }
    #[inline]
    fn from_c_repr((a, b, c, d, e, f, g, h, i, j): Self::CRepr) -> Self {
        (A::from_c_repr(a), B::from_c_repr(b), C::from_c_repr(c), D::from_c_repr(d), E::from_c_repr(e), F::from_c_repr(f), G::from_c_repr(g), H::from_c_repr(h), I::from_c_repr(i), J::from_c_repr(j))
    }
}


impl<ReprRust> ConvRepr for Box<ReprRust> where ReprRust: ConvRepr {
    type CRepr = *mut ReprRust::CRepr;

    #[inline]
    fn into_c_repr(self) -> Self::CRepr {
        Box::into_raw(Box::new((*self).into_c_repr()))
    }
    #[inline]
    fn from_c_repr(c: Self::CRepr) -> Self {
        if c.is_null() {
            unsafe { Box::from_raw(std::ptr::null_mut()) }
        } else {
            Box::new(ReprRust::from_c_repr(unsafe { *Box::from_raw(c) }))
        }
    }
}

impl<ReprRust> ConvRepr for Option<Box<ReprRust>> where ReprRust: ConvRepr {
    type CRepr = *mut ReprRust::CRepr;

    #[inline]
    fn into_c_repr(self) -> Self::CRepr {
        if let Some(x) = self {
            x.into_c_repr()
        } else {
            ::std::ptr::null_mut()
        }
    }

    #[inline]
    fn from_c_repr(c: Self::CRepr) -> Self {
        if c.is_null() {
            None
        } else {
            Some(Box::new(ReprRust::from_c_repr(unsafe { *Box::from_raw(c) })))
        }
    }
}

impl ConvRepr for String {
    type CRepr = C_String;

    #[inline]
    fn into_c_repr(self) -> Self::CRepr {
        C_String::from_string(self)
    }

    #[inline]
    fn from_c_repr(c: Self::CRepr) -> Self {
        c.into_string()
    }
}


impl<ReprRust> ConvRepr for Vec<ReprRust> where ReprRust: ConvRepr + Any {
    type CRepr = FfiArray<ReprRust::CRepr>;

    #[inline]
    fn into_c_repr(self) -> Self::CRepr {
        if self.is_empty() {
            return Self::CRepr::null();
        }
        if TypeId::of::<ReprRust>() == TypeId::of::<ReprRust::CRepr>() {
            FfiArray {
                len: self.len(),
                cap: self.capacity(),
                ptr: self.leak().as_mut_ptr() as *mut ReprRust::CRepr,
            }
        } else {
            Self::CRepr::from_vec(self.into_iter()
                .map(|v| v.into_c_repr())
                .collect::<Vec<ReprRust::CRepr>>()
            )
        }
    }
    #[inline]
    fn from_c_repr(c: Self::CRepr) -> Self {
        if c.is_empty() {
            return Vec::new();
        }
        if TypeId::of::<ReprRust>() == TypeId::of::<ReprRust::CRepr>() {
            let mut v = unsafe { Vec::from_raw_parts(c.ptr as *mut ReprRust, c.len, c.cap) };
            v.shrink_to_fit();
            v
        } else {
            c.into_vec().into_iter().map(|v| ReprRust::from_c_repr(v)).collect()
        }
    }
}


impl<ReprRustK, ReprRustV> ConvRepr for Map<ReprRustK, ReprRustV>
    where
        ReprRustK: ConvRepr,
        ReprRustV: ConvRepr,
{
    type CRepr = C_Map<ReprRustK::CRepr, ReprRustV::CRepr>;

    #[inline]
    fn into_c_repr(self) -> Self::CRepr {
        if self.0.is_empty() {
            return Self::CRepr::null();
        }
        Self::CRepr::from_vec(self.0.into_iter()
            .map(|kv| MapEntry { key: kv.key.into_c_repr(), value: kv.value.into_c_repr() })
            .collect::<Vec<MapEntry<ReprRustK::CRepr, ReprRustV::CRepr>>>()
        )
    }
    #[inline]
    fn from_c_repr(c: Self::CRepr) -> Self {
        if c.is_empty() {
            return Map(Vec::new());
        }
        Map(c.into_vec().into_iter().map(|kv| MapEntry { key: ReprRustK::from_c_repr(kv.key), value: ReprRustV::from_c_repr(kv.value) }).collect())
    }
}

impl<ReprRustK, ReprRustV> ConvRepr for HashMap<ReprRustK, ReprRustV>
    where
        ReprRustK: ConvRepr + PartialEq + Eq + Hash,
        ReprRustV: ConvRepr,
{
    type CRepr = C_Map<ReprRustK::CRepr, ReprRustV::CRepr>;

    #[inline]
    fn into_c_repr(self) -> Self::CRepr {
        if self.is_empty() {
            return Self::CRepr::null();
        }
        Self::CRepr::from_vec(self.into_iter()
            .map(|(k, v)| MapEntry { key: k.into_c_repr(), value: v.into_c_repr() })
            .collect::<Vec<MapEntry<ReprRustK::CRepr, ReprRustV::CRepr>>>()
        )
    }
    #[inline]
    fn from_c_repr(c: Self::CRepr) -> Self {
        if c.is_empty() {
            return Self::new();
        }
        Map::<ReprRustK, ReprRustV>(c.into_vec()
            .into_iter()
            .map(|kv| MapEntry { key: ReprRustK::from_c_repr(kv.key), value: ReprRustV::from_c_repr(kv.value) })
            .collect::<Vec<MapEntry<ReprRustK, ReprRustV>>>()
        )
            .into_hash_map()
    }
}


impl<ReprRust> ConvRepr for HashSet<ReprRust>
    where
        ReprRust: ConvRepr + Eq + Hash,
{
    type CRepr = C_Set<ReprRust::CRepr>;

    #[inline]
    fn into_c_repr(self) -> Self::CRepr {
        if self.is_empty() {
            return Self::CRepr::null();
        }
        Self::CRepr::from_vec(self.into_iter()
            .map(|v| v.into_c_repr())
            .collect::<Vec<ReprRust::CRepr>>()
        )
    }
    #[inline]
    fn from_c_repr(c: Self::CRepr) -> Self {
        if c.is_empty() {
            return Self::new();
        }
        HashSet::<ReprRust>::from_iter(c.into_vec()
            .into_iter()
            .map(|v| ReprRust::from_c_repr(v)))
    }
}

macro_rules! impl_scalar_conv_c_repr {
    () => {};
    ($ty:ty; $($tail:tt)*) => {
        impl ConvRepr for $ty {
            type CRepr = $ty;
            #[inline]
            fn into_c_repr(self) -> Self::CRepr {
                self
            }
            #[inline]
            fn from_c_repr(c: Self::CRepr) -> Self {
                c
            }
        }
        impl_scalar_conv_c_repr!($($tail)*);
    }
}

impl_scalar_conv_c_repr!((); bool; i8; i16; i32; i64; i128; u8; u16; u32; u64; u128; f32; f64;);


#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct FfiArray<T> {
    pub ptr: *mut T,
    pub len: usize,
    pub cap: usize,
}

impl<T> FfiArray<T> {
    pub fn null() -> Self {
        Self {
            ptr: std::ptr::null_mut(),
            len: 0,
            cap: 0,
        }
    }
    pub fn from_vec(v: Vec<T>) -> Self {
        let s = FfiArray {
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

type C_Bytes = FfiArray<u8>;

impl FfiArray<u8> {
    #[inline]
    pub fn from_bytes(v: Vec<u8>) -> Self {
        Self::from_vec(v)
    }
    #[inline]
    pub fn into_bytes(self) -> Vec<u8> {
        self.into_vec()
    }
}

pub type C_String = FfiArray<u8>;

impl FfiArray<u8> {
    #[inline]
    pub fn from_string(s: String) -> Self {
        Self::from_vec(s.into_bytes())
    }
    #[inline]
    pub fn into_string(self) -> String {
        unsafe { String::from_utf8_unchecked(self.into_vec()) }
    }
}

pub type C_Map<K, V> = FfiArray<MapEntry<K, V>>;

impl<K: Eq + Hash + Ord, V> FfiArray<MapEntry<K, V>> {
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

#[derive(Clone, Debug, PartialEq)]
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

type C_Set<T> = FfiArray<T>;

impl<T: Eq + Hash> FfiArray<T> {
    #[inline]
    pub fn from_hash_set(s: HashSet<T>) -> Self {
        Self::from_vec(s.into_iter().collect())
    }
    #[inline]
    fn into_hash_set(self) -> HashSet<T> {
        HashSet::from_iter(self.into_vec().into_iter())
    }
}


pub struct GoFfiResult<RET, ARGS: ConvRepr> {
    ret: RET,
    c_args: *mut ARGS::CRepr,
    c_ret_ptr: usize,
    c_free_fn: unsafe extern "C" fn(usize),
}

impl<RET, ARGS: ConvRepr> GoFfiResult<RET, ARGS> {
    pub fn new(ret: RET, c_args: *mut ARGS::CRepr, c_ret_ptr: usize, c_free_fn: unsafe extern "C" fn(usize)) -> Self {
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

impl<RET, ARGS: ConvRepr> Drop for GoFfiResult<RET, ARGS> {
    fn drop(&mut self) {
        let Self { c_args, c_ret_ptr, c_free_fn, .. } = self;
        let _ = ARGS::from_c_repr(*unsafe { Box::from_raw(c_args.clone()) });
        unsafe { c_free_fn(c_ret_ptr.clone()) };
    }
}
