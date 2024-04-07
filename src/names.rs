use std::{
    hash::{Hash, Hasher},
    rc::Rc,
    sync::atomic::AtomicI32,
    sync::atomic::Ordering::Relaxed,
};

#[derive(Debug, Clone)]
pub struct NameNode {
    pub name: String,
    pub id: i32,
}

static STAMP: AtomicI32 = AtomicI32::new(0);

pub type Name = Rc<NameNode>;

impl NameNode {
    pub fn create(s: String) -> Self {
        NameNode {
            name: s,
            id: STAMP.fetch_add(1, Relaxed),
        }
    }
}

impl Hash for NameNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for NameNode {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for NameNode {}
