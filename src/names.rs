use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
    rc::*,
    sync::atomic::{AtomicI32, Ordering::Relaxed},
};

#[derive(Debug, Clone)]
pub struct Name {
    pub name: String,
    pub id: i32,
}

static STAMP: AtomicI32 = AtomicI32::new(0);

impl Name {
    pub fn create(s: String) -> Rc<Self> {
        Rc::new(Name {
            name: s,
            id: STAMP.fetch_add(1, Relaxed),
        })
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
