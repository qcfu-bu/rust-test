use std::{
    cmp::Ordering,
    fmt::Debug,
    hash::{DefaultHasher, Hash, Hasher},
    rc::*,
    sync::atomic::{AtomicI32, Ordering::Relaxed},
};

#[derive(Clone)]
pub struct Name {
    pub name: Rc<String>,
    pub id: u64,
}

impl Debug for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}_{}", self.name, self.id))
    }
}

static STAMP: AtomicI32 = AtomicI32::new(0);

impl Name {
    pub fn create(s: String) -> Self {
        let mut state = DefaultHasher::new();
        s.hash(&mut state);
        Name {
            name: Rc::new(s),
            id: state.finish(),
        }
    }
}

impl Hash for Name {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialOrd for Name {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl Ord for Name {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialEq for Name {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Name {}
