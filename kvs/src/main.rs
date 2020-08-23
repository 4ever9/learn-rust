use kv::KVStore;

mod kv;

fn main() {
    let mut kvs = KVStore::new();
    kvs.set("key".to_string(), "value".to_string());
    let value = kvs.get("key".to_string());
    assert_eq!(Some("value".to_owned()), value);

    kvs.remove("key".to_string());
    assert_eq!(None, kvs.get("key".to_string()));
}