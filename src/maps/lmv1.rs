use std::{
    marker::PhantomData,
    mem::transmute,
    ptr::{slice_from_raw_parts, slice_from_raw_parts_mut},
};

use crate::maps::Finder;

#[derive(Debug, Clone, Copy)]
pub struct Entry {
    pub key_start: u32,
    pub key_len: u16,
    pub val_len: u16,
    _private: PhantomData<()>,
}
#[derive(Debug)]
pub enum LinearMapError {
    Overflow,
}

pub struct LinearMap {
    raw: Vec<u8>,
    entries: Vec<Entry>,
}

impl LinearMap {
    pub const MAX_BYTE_LEN: usize = u32::MAX as usize;
    pub const MAX_KEY_VAL_LEN: usize = u16::MAX as usize;
    const DEFAULT_ENTRIES: u16 = 17;
    const DEFAULT_ENTRY_SIZE: u16 = 7;

    pub fn new() -> Self {
        Self::new_with(Self::DEFAULT_ENTRIES, Self::DEFAULT_ENTRY_SIZE)
    }

    pub fn new_with(n_entries: u16, entry_size: u16) -> Self {
        let entry_bytes = n_entries as usize * entry_size as usize;
        Self {
            raw: Vec::with_capacity(entry_bytes),
            entries: Vec::with_capacity(n_entries as _),
        }
    }

    pub fn append(&mut self, key: &[u8], val: &[u8]) -> Result<(), LinearMapError> {
        // Overflow can never occure
        let pack_size = unsafe { key.len().unchecked_add(val.len()) };
        let new_len = pack_size + self.raw.len();

        // Conditions Checks
        if (pack_size > Self::MAX_BYTE_LEN)
            | (new_len > Self::MAX_BYTE_LEN)
            | (key.len() > Self::MAX_KEY_VAL_LEN)
            | (val.len() > Self::MAX_KEY_VAL_LEN)
        {
            return Err(LinearMapError::Overflow);
        }

        // Creating Entry
        let entry = Entry {
            key_start: self.raw.len() as _,
            key_len: key.len() as _,
            val_len: val.len() as _,
            _private: PhantomData,
        };

        // Extending the Bytes
        self.raw.extend_from_slice(key);
        self.raw.extend_from_slice(val);

        // Updating entries store
        self.entries.push(entry);

        Ok(())
    }

    #[inline(always)]
    pub fn get_map_entry(&self, key: &[u8]) -> Option<Entry> {
        for entry in &self.entries {
            unsafe {
                let key_ptr = self.raw.as_ptr().add(entry.key_start as usize);

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
            let base = self.raw.as_mut_ptr();

            let key_start = e.key_start as usize;
            let val_start = key_start.unchecked_add(e.key_len as usize);

            let key_ptr = base.add(key_start);
            let val_ptr = base.add(val_start);

            let key = &mut *slice_from_raw_parts_mut(key_ptr, e.key_len as usize);

            let val = &mut *slice_from_raw_parts_mut(val_ptr, e.val_len as usize);

            (key, val)
        }
    }

    #[inline(always)]
    pub unsafe fn get_by_entry(&self, e: Entry) -> (&[u8], &[u8]) {
        unsafe {
            let base = self.raw.as_ptr();

            let key_start = e.key_start as usize;
            let val_start = key_start.unchecked_add(e.key_len as usize);

            let key_ptr = base.add(key_start);
            let val_ptr = base.add(val_start);

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
}

impl Finder for LinearMap {
    type KeyType = String;
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
    fn test_append_and_get() {
        let mut map = LinearMap::new();

        map.append(b"name", b"lokesh").unwrap();
        map.append(b"lang", b"rust").unwrap();

        assert_eq!(map.get(b"name"), Some(&b"lokesh"[..]));
        assert_eq!(map.get(b"lang"), Some(&b"rust"[..]));
    }

    #[test]
    fn test_contains() {
        let mut map = LinearMap::new();

        map.append(b"a", b"1").unwrap();

        assert!(map.contains(b"a"));
        assert!(!map.contains(b"b"));
    }

    #[test]
    fn test_entry() {
        let mut map = LinearMap::new();

        map.append(b"key", b"value").unwrap();

        let (k, v) = map.entry(b"key").unwrap();

        assert_eq!(k, b"key");
        assert_eq!(v, b"value");
    }

    #[test]
    fn test_get_mut() {
        let mut map = LinearMap::new();

        map.append(b"hello", b"world").unwrap();

        {
            let val = map.get_mut(b"hello").unwrap();

            val.copy_from_slice(b"earth");
        }

        assert_eq!(map.get(b"hello"), Some(&b"earth"[..]));
    }

    #[test]
    fn test_multiple_entries() {
        let mut map = LinearMap::new();

        for i in 0..100 {
            let key = format!("key-{i}");
            let val = format!("val-{i}");

            map.append(key.as_bytes(), val.as_bytes()).unwrap();
        }

        for i in 0..100 {
            let key = format!("key-{i}");
            let val = format!("val-{i}");

            assert_eq!(map.get(key.as_bytes()), Some(val.as_bytes()));
        }
    }

    #[test]
    fn test_empty_key() {
        let mut map = LinearMap::new();

        map.append(b"", b"empty-key").unwrap();

        assert_eq!(map.get(b""), Some(&b"empty-key"[..]));
    }

    #[test]
    fn test_empty_value() {
        let mut map = LinearMap::new();

        map.append(b"empty", b"").unwrap();

        assert_eq!(map.get(b"empty"), Some(&b""[..]));
    }

    #[test]
    fn test_duplicate_keys_returns_first() {
        let mut map = LinearMap::new();

        map.append(b"dup", b"first").unwrap();
        map.append(b"dup", b"second").unwrap();

        assert_eq!(map.get(b"dup"), Some(&b"first"[..]));
    }

    #[test]
    fn test_not_found() {
        let mut map = LinearMap::new();

        map.append(b"a", b"b").unwrap();

        assert_eq!(map.get(b"missing"), None);
        assert_eq!(map.entry(b"missing"), None);
    }

    #[test]
    fn test_large_key_value() {
        let mut map = LinearMap::new();

        let key = vec![b'k'; 1024];
        let val = vec![b'v'; 4096];

        map.append(&key, &val).unwrap();

        assert_eq!(map.get(&key), Some(val.as_slice()));
    }

    #[test]
    fn test_get_map_entry() {
        let mut map = LinearMap::new();

        map.append(b"alpha", b"beta").unwrap();

        let entry = map.get_map_entry(b"alpha");

        assert!(entry.is_some());

        let entry = entry.unwrap();

        assert_eq!(entry.key_len, 5);
        assert_eq!(entry.val_len, 4);
    }

    #[test]
    fn test_get_by_entry() {
        let mut map = LinearMap::new();

        map.append(b"k1", b"v1").unwrap();

        let entry = map.get_map_entry(b"k1").unwrap();

        unsafe {
            let (k, v) = map.get_by_entry(entry);

            assert_eq!(k, b"k1");
            assert_eq!(v, b"v1");
        }
    }

    #[test]
    fn test_get_by_entry_mut() {
        let mut map = LinearMap::new();

        map.append(b"abc", b"123").unwrap();

        let entry = map.get_map_entry(b"abc").unwrap();

        unsafe {
            let (_, val) = map.get_by_entry_mut(entry);

            val.copy_from_slice(b"999");
        }

        assert_eq!(map.get(b"abc"), Some(&b"999"[..]));
    }

    #[test]
    fn test_binary_data() {
        let mut map = LinearMap::new();

        let key = [0, 1, 2, 3];
        let val = [255, 254, 253, 252];

        map.append(&key, &val).unwrap();

        assert_eq!(map.get(&key), Some(&val[..]));
    }

    #[test]
    fn test_zero_entries() {
        let map = LinearMap::new();

        assert_eq!(map.get(b"anything"), None);
        assert!(!map.contains(b"anything"));
    }
}
