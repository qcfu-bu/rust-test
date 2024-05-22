use std::{
    hash::{Hash, Hasher},
    sync::atomic::AtomicI32,
    sync::atomic::Ordering::Relaxed,
};

use bumpalo::Bump;

#[derive(Debug, Clone)]
pub struct Name {
    pub name: String,
    pub id: i32,
}

static STAMP: AtomicI32 = AtomicI32::new(0);

impl Name {
    pub fn create(s: String, bump: &Bump) -> &Self {
        bump.alloc(Name {
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

impl PartialEq for Name {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Name {}
