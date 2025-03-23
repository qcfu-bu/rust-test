use rpds::List;
pub type ListMap<K, V> = List<(K, V)>;

pub fn nil<K, V>() -> ListMap<K, V> {
    List::new()
}

pub fn cons<K, V>(x: (K, V), xs: ListMap<K, V>) -> ListMap<K, V> {
    xs.push_front(x)
}

pub fn find<K, V>(k: K, xs: ListMap<K, V>) -> Option<V>
where
    K: PartialEq,
    V: Clone,
{
    let mut res: Option<V> = None;
    for (k0, v) in &xs {
        if k == *k0 {
            res = Some(v.clone());
            break;
        }
    }
    res
}
