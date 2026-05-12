use std::{
    marker::PhantomData,
    mem::transmute,
    ptr::{slice_from_raw_parts, slice_from_raw_parts_mut},
};

use crate::maps::Finder;

#[repr(C, align(4))]
#[derive(Debug, Clone, Copy)]
pub struct Entry {
    pub key_start: u32,
    pub val_start: u32,
    pub key_len: u16,
    pub val_len: u16,
    _private: PhantomData<()>,
}

#[derive(Debug)]
pub enum LinearMapError {
    Overflow,
}

pub struct LinearMapV2 {
    raw_keys: Vec<u8>,
    raw_values: Vec<u8>,
    entries: Vec<Entry>,
}

impl LinearMapV2 {
    pub const MAX_BYTE_LEN: usize = u32::MAX as usize;
    pub const MAX_KEY_VAL_SIZE: usize = u16::MAX as usize;

    /// Arbitrary defaults from logs because
    /// I'm lazy & Int & Dumb all at the same time like Shodingers-cat
    /// So summoning magic numbers and calling it engineering.
    const DEFAULT_ENTRIES: u16 = 17;

    const DEFAULT_ENTRY_KEY_SIZE: u16 = 7;
    const DEFAULT_ENTRY_VAL_SIZE: u16 = 12;

    pub fn new() -> Self {
        Self::new_with(
            Self::DEFAULT_ENTRIES,
            Self::DEFAULT_ENTRY_KEY_SIZE,
            Self::DEFAULT_ENTRY_VAL_SIZE,
        )
    }
    pub fn new_with(n_entries: u16, key_size: u16, val_size: u16) -> Self {
        Self {
            raw_keys: Vec::with_capacity(n_entries as usize * key_size as usize),
            raw_values: Vec::with_capacity(n_entries as usize * val_size as usize),
            entries: Vec::with_capacity(n_entries as usize),
        }
    }

    pub fn append(&mut self, key: &[u8], val: &[u8]) -> Result<(), LinearMapError> {
        // Practically its impossible to do an overflow for an usize/u64 with len so we assume its same
        let new_key_size = unsafe { key.len().unchecked_add(self.raw_keys.len()) };
        // Practically its impossible to do an overflow for an usize/u64 with len so we assume its same
        let new_val_size = unsafe { val.len().unchecked_add(self.raw_values.len()) };

        // Conditions Checks
        if (new_key_size > Self::MAX_BYTE_LEN)
            | (new_val_size > Self::MAX_BYTE_LEN)
            | (key.len() > Self::MAX_KEY_VAL_SIZE)
            | (val.len() > Self::MAX_KEY_VAL_SIZE)
        {
            return Err(LinearMapError::Overflow);
        }

        // Creating Entry
        let entry = Entry {
            key_start: self.raw_keys.len() as _,
            val_start: self.raw_values.len() as _,
            key_len: key.len() as _,
            val_len: val.len() as _,
            _private: PhantomData,
        };

        // Extending the Bytes
        self.raw_keys.extend_from_slice(key);
        self.raw_values.extend_from_slice(val);

        // Updating entries store
        self.entries.push(entry);

        Ok(())
    }

    #[inline(always)]
    pub fn get_map_entry(&self, key: &[u8]) -> Option<Entry> {
        for entry in &self.entries {
            unsafe {
                let key_ptr = self.raw_keys.as_ptr().add(entry.key_start as usize);

                let key_slice = &*slice_from_raw_parts(key_ptr, entry.key_len as usize);

                if key_slice == key {
                    return Some(*entry);
                }
            }
        }

        None
    }

    #[inline(always)]
    pub unsafe fn get_by_entry_mut(&mut self, e: Entry) -> (&mut [u8], &mut [u8]) {
        unsafe {
            let key_ptr = self.raw_keys.as_mut_ptr().add(e.key_start as usize);

            let val_ptr = self.raw_values.as_mut_ptr().add(e.val_start as usize);

            let key = &mut *slice_from_raw_parts_mut(key_ptr, e.key_len as usize);

            let val = &mut *slice_from_raw_parts_mut(val_ptr, e.val_len as usize);

            (key, val)
        }
    }

    #[inline(always)]
    pub unsafe fn get_by_entry(&self, e: Entry) -> (&[u8], &[u8]) {
        unsafe {
            let key_ptr = self.raw_keys.as_ptr().add(e.key_start as usize);

            let val_ptr = self.raw_values.as_ptr().add(e.val_start as usize);

            let key = &*slice_from_raw_parts(key_ptr, e.key_len as usize);

            let val = &*slice_from_raw_parts(val_ptr, e.val_len as usize);

            (key, val)
        }
    }

    #[inline(always)]
    pub fn entry(&self, key: &[u8]) -> Option<(&[u8], &[u8])> {
        match self.get_map_entry(key) {
            Some(entry) => unsafe { Some(self.get_by_entry(entry)) },
            None => None,
        }
    }

    #[inline(always)]
    pub fn contains(&self, key: &[u8]) -> bool {
        self.get_map_entry(key).is_some()
    }

    #[inline(always)]
    pub fn get(&self, key: &[u8]) -> Option<&[u8]> {
        match self.get_map_entry(key) {
            Some(entry) => unsafe {
                let (_, val) = self.get_by_entry(entry);
                Some(val)
            },
            None => None,
        }
    }

    #[inline(always)]
    pub fn get_mut(&mut self, key: &[u8]) -> Option<&mut [u8]> {
        let entry = self.get_map_entry(key)?;

        unsafe {
            let (_, val) = self.get_by_entry_mut(entry);
            Some(val)
        }
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    #[inline(always)]
    pub fn capacity(&self) -> usize {
        self.entries.capacity()
    }

    #[inline(always)]
    pub fn raw_keys_len(&self) -> usize {
        self.raw_keys.len()
    }

    #[inline(always)]
    pub fn raw_values_len(&self) -> usize {
        self.raw_values.len()
    }

    #[inline(always)]
    pub fn raw_len(&self) -> usize {
        // Safe as (u32::MAX + u32::MAX) < usize::MAX
        unsafe { self.raw_keys.len().unchecked_add(self.raw_values.len()) }
    }

    #[inline(always)]
    pub fn iter(&self) -> impl Iterator<Item = (&[u8], &[u8])> {
        self.entries
            .iter()
            .map(move |e| unsafe { self.get_by_entry(*e) })
    }

    #[inline(always)]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&mut [u8], &mut [u8])> {
        let key_base = self.raw_keys.as_mut_ptr();
        let val_base = self.raw_values.as_mut_ptr();

        self.entries.iter().map(move |e| unsafe {
            let key_ptr = key_base.add(e.key_start as usize);

            let val_ptr = val_base.add(e.val_start as usize);

            let key = &mut *slice_from_raw_parts_mut(key_ptr, e.key_len as usize);

            let val = &mut *slice_from_raw_parts_mut(val_ptr, e.val_len as usize);

            (key, val)
        })
    }

    #[inline(always)]
    pub fn keys(&self) -> impl Iterator<Item = &[u8]> {
        self.entries.iter().map(move |e| unsafe {
            let (k, _) = self.get_by_entry(*e);
            k
        })
    }

    #[inline(always)]
    pub fn values(&self) -> impl Iterator<Item = &[u8]> {
        self.entries.iter().map(move |e| unsafe {
            let (_, v) = self.get_by_entry(*e);
            v
        })
    }

    #[inline(always)]
    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut [u8]> {
        let val_base = self.raw_values.as_mut_ptr();

        self.entries.iter().map(move |e| unsafe {
            let val_ptr = val_base.add(e.val_start as usize);

            &mut *slice_from_raw_parts_mut(val_ptr, e.val_len as usize)
        })
    }

    #[inline(always)]
    pub fn entry_by_idx(&self, idx: usize) -> Option<(&[u8], &[u8])> {
        let entry = *self.entries.get(idx)?;

        unsafe { Some(self.get_by_entry(entry)) }
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        self.raw_keys.clear();
        self.raw_values.clear();
        self.entries.clear();
    }

    #[inline(always)]
    pub fn reserve_exact(&mut self, entries: usize, key_bytes: usize, val_bytes: usize) {
        self.entries.reserve_exact(entries);
        self.raw_keys.reserve_exact(key_bytes);
        self.raw_values.reserve_exact(val_bytes);
    }

    #[inline(always)]
    pub fn shrink_to_fit(&mut self) {
        self.entries.shrink_to_fit();
        self.raw_keys.shrink_to_fit();
        self.raw_values.shrink_to_fit();
    }
}

impl Finder for LinearMapV2 {
    type KeyType = &'static str;
    type ValType<'a> = &'a str;

    fn new() -> Self {
        Self::new()
    }

    fn find<'a>(&self, data: Self::KeyType) -> Option<Self::ValType<'a>> {
        self.get(data.as_bytes())
            .map(move |s| unsafe { transmute(str::from_utf8_unchecked(s)) })
    }

    fn push<'a>(&mut self, key: Self::KeyType, val: Self::ValType<'a>) {
        self.append(key.as_bytes(), val.as_bytes()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_map_is_empty() {
        let map = LinearMapV2::new();

        assert_eq!(map.len(), 0);
        assert!(map.is_empty());
        assert_eq!(map.raw_len(), 0);
        assert_eq!(map.raw_keys_len(), 0);
        assert_eq!(map.raw_values_len(), 0);
    }

    #[test]
    fn test_append_and_get() {
        let mut map = LinearMapV2::new();

        map.append(b"hello", b"world").unwrap();

        assert_eq!(map.len(), 1);
        assert_eq!(map.get(b"hello"), Some(&b"world"[..]));
        assert_eq!(map.get(b"missing"), None);
    }

    #[test]
    fn test_contains() {
        let mut map = LinearMapV2::new();

        map.append(b"abc", b"123").unwrap();

        assert!(map.contains(b"abc"));
        assert!(!map.contains(b"xyz"));
    }

    #[test]
    fn test_entry() {
        let mut map = LinearMapV2::new();

        map.append(b"k1", b"v1").unwrap();

        let (k, v) = map.entry(b"k1").unwrap();

        assert_eq!(k, b"k1");
        assert_eq!(v, b"v1");
    }

    #[test]
    fn test_get_mut() {
        let mut map = LinearMapV2::new();

        map.append(b"key", b"value").unwrap();

        let val = map.get_mut(b"key").unwrap();

        val.copy_from_slice(b"VALUE");

        assert_eq!(map.get(b"key"), Some(&b"VALUE"[..]));
    }

    #[test]
    fn test_iter() {
        let mut map = LinearMapV2::new();

        map.append(b"a", b"1").unwrap();
        map.append(b"b", b"2").unwrap();
        map.append(b"c", b"3").unwrap();

        let collected: Vec<(Vec<u8>, Vec<u8>)> =
            map.iter().map(|(k, v)| (k.to_vec(), v.to_vec())).collect();

        assert_eq!(
            collected,
            vec![
                (b"a".to_vec(), b"1".to_vec()),
                (b"b".to_vec(), b"2".to_vec()),
                (b"c".to_vec(), b"3".to_vec()),
            ]
        );
    }

    #[test]
    fn test_iter_mut() {
        let mut map = LinearMapV2::new();

        map.append(b"a", b"111").unwrap();
        map.append(b"b", b"222").unwrap();

        for (_, val) in map.iter_mut() {
            for byte in val {
                *byte = b'x';
            }
        }

        assert_eq!(map.get(b"a"), Some(&b"xxx"[..]));
        assert_eq!(map.get(b"b"), Some(&b"xxx"[..]));
    }

    #[test]
    fn test_keys() {
        let mut map = LinearMapV2::new();

        map.append(b"k1", b"v1").unwrap();
        map.append(b"k2", b"v2").unwrap();

        let keys: Vec<Vec<u8>> = map.keys().map(|k| k.to_vec()).collect();

        assert_eq!(keys, vec![b"k1".to_vec(), b"k2".to_vec(),]);
    }

    #[test]
    fn test_values() {
        let mut map = LinearMapV2::new();

        map.append(b"k1", b"v1").unwrap();
        map.append(b"k2", b"v2").unwrap();

        let vals: Vec<Vec<u8>> = map.values().map(|v| v.to_vec()).collect();

        assert_eq!(vals, vec![b"v1".to_vec(), b"v2".to_vec(),]);
    }

    #[test]
    fn test_values_mut() {
        let mut map = LinearMapV2::new();

        map.append(b"a", b"111").unwrap();
        map.append(b"b", b"222").unwrap();

        for val in map.values_mut() {
            val[0] = b'z';
        }

        assert_eq!(map.get(b"a"), Some(&b"z11"[..]));
        assert_eq!(map.get(b"b"), Some(&b"z22"[..]));
    }

    #[test]
    fn test_entry_by_idx() {
        let mut map = LinearMapV2::new();

        map.append(b"first", b"111").unwrap();
        map.append(b"second", b"222").unwrap();

        let (k, v) = map.entry_by_idx(1).unwrap();

        assert_eq!(k, b"second");
        assert_eq!(v, b"222");

        assert!(map.entry_by_idx(100).is_none());
    }

    #[test]
    fn test_clear() {
        let mut map = LinearMapV2::new();

        map.append(b"a", b"1").unwrap();
        map.append(b"b", b"2").unwrap();

        map.clear();

        assert_eq!(map.len(), 0);
        assert!(map.is_empty());
        assert_eq!(map.raw_len(), 0);
    }

    #[test]
    fn test_reserve_exact() {
        let mut map = LinearMapV2::new();

        map.reserve_exact(100, 1000, 2000);

        assert!(map.capacity() >= 100);
    }

    #[test]
    fn test_shrink_to_fit() {
        let mut map = LinearMapV2::new();

        map.reserve_exact(100, 1000, 1000);

        map.append(b"a", b"b").unwrap();

        map.shrink_to_fit();

        assert!(map.capacity() >= 1);
    }

    #[test]
    fn test_multiple_entries_same_key_content() {
        let mut map = LinearMapV2::new();

        map.append(b"dup", b"111").unwrap();
        map.append(b"dup", b"222").unwrap();

        // current implementation returns first match
        assert_eq!(map.get(b"dup"), Some(&b"111"[..]));
    }

    #[test]
    fn test_empty_key_and_value() {
        let mut map = LinearMapV2::new();

        map.append(b"", b"").unwrap();

        assert_eq!(map.get(b""), Some(&b""[..]));

        let (k, v) = map.entry(b"").unwrap();

        assert_eq!(k, b"");
        assert_eq!(v, b"");
    }

    #[test]
    fn test_large_values() {
        let mut map = LinearMapV2::new();

        let key = vec![b'k'; 1024];
        let val = vec![b'v'; 4096];

        map.append(&key, &val).unwrap();

        assert_eq!(map.get(&key), Some(val.as_slice()));
    }

    #[test]
    fn test_get_map_entry() {
        let mut map = LinearMapV2::new();

        map.append(b"abc", b"123").unwrap();

        let entry = map.get_map_entry(b"abc");

        assert!(entry.is_some());

        let entry = entry.unwrap();

        assert_eq!(entry.key_len, 3);
        assert_eq!(entry.val_len, 3);
    }

    #[test]
    fn test_unsafe_get_by_entry() {
        let mut map = LinearMapV2::new();

        map.append(b"hello", b"world").unwrap();

        let entry = map.get_map_entry(b"hello").unwrap();

        unsafe {
            let (k, v) = map.get_by_entry(entry);

            assert_eq!(k, b"hello");
            assert_eq!(v, b"world");
        }
    }

    #[test]
    fn test_unsafe_get_by_entry_mut() {
        let mut map = LinearMapV2::new();

        map.append(b"hello", b"world").unwrap();

        let entry = map.get_map_entry(b"hello").unwrap();

        unsafe {
            let (_, v) = map.get_by_entry_mut(entry);

            v.copy_from_slice(b"WORLD");
        }

        assert_eq!(map.get(b"hello"), Some(&b"WORLD"[..]));
    }
}
