// #[cfg(test)]
// mod tests {
//     use crate::store::KvStore;

//     #[test]
//     fn test_set_get() {
//         let store = KvStore::new();
//         store.set("foo".to_string(), "bar".to_string());
//         assert_eq!(store.get("foo"), Some("bar".to_string()));
//     }

//     #[test]
//     fn test_delete() {
//         let store = KvStore::new();
//         store.set("foo".to_string(), "bar".to_string());
//         assert!(store.delete("foo"));
//         assert_eq!(store.get("foo"), None);
//     }
// }

use quickcheck::quickcheck;

quickcheck! {
    fn prop_set_get(key: String, value: String) -> bool {
        let store = KvStore::new();
        store.set(key.clone(), value.clone());
        store.get(&key) == Some(value)
    }
}

