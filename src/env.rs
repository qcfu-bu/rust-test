use std::rc::*;

#[derive(Debug, Clone)]
pub enum List<A> {
    Nil,
    Cons(A, Rc<List<A>>),
}

pub fn nil<A>() -> Rc<List<A>> {
    Rc::new(List::Nil)
}

pub fn cons<A>(x: A, xs: Rc<List<A>>) -> Rc<List<A>> {
    Rc::new(List::Cons(x, xs))
}

pub fn find<A, B>(k: A, mut xs: Rc<List<(A, Rc<B>)>>) -> Option<Rc<B>>
where
    A: PartialEq,
{
    let mut res = None;
    while let List::Cons((k0, x), xs0) = &*xs {
        if k == *k0 {
            res = Some(x.clone());
            break;
        }
        xs = xs0.clone()
    }
    res
}
