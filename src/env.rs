use bumpalo::Bump;

#[derive(Debug, Clone)]
pub enum List<'a, A> {
    Nil,
    Cons(A, &'a List<'a, A>),
}

pub fn nil<'a, A>(bump: &'a Bump) -> &'a List<'a, A> {
    bump.alloc(List::Nil)
}

pub fn cons<'a, A>(x: A, xs: &'a List<A>, bump: &'a Bump) -> &'a List<'a, A> {
    bump.alloc(List::Cons(x, xs))
}

pub fn find<'a, A, B>(k: A, mut xs: &'a List<'a, (A, B)>) -> Option<&'a B>
where
    A: PartialEq,
{
    let mut res = None;
    while let List::Cons((k0, x), xs0) = xs {
        if k == *k0 {
            res = Some(x);
            break;
        }
        xs = xs0
    }
    res
}
