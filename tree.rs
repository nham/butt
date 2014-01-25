use std::fmt::{Default, Formatter};

#[deriving(Eq, Clone)]
pub enum Tree<T> {
    Leaf(T),
    Branch(~[Tree<T>])
}

impl<T> Tree<T> {
    pub fn is_leaf(&self) -> bool {
        match *self {
            Leaf(_) => true,
            _       => false
        }
    }

    pub fn is_branch(&self) -> bool {
        !self.is_leaf()
    }

    pub fn get_ref_leaf<'a>(&'a self) -> &'a T {
        match *self {
            Leaf(ref val) => val,
            _         => fail!("called Tree<T>::unwrap_leaf() on Branch()"),
        }
    }

    pub fn unwrap_branch(self) -> ~[Tree<T>] {
        match self {
            Branch(val) => val,
            _         => fail!("called Tree<T>::unwrap_branch() on Branch()"),
        }
    }

}

impl<T: Default> Default for Tree<T> {
    fn fmt(v: &Tree<T>, f: &mut Formatter) {
        match *v {
            Branch(ref vec) => write!(f.buf, "Branch{}", *vec),
            Leaf(ref val) => write!(f.buf, "Leaf({})", *val)
        }
    }
}

impl<T: Default> Default for ~[Tree<T>] {
    fn fmt(v: &~[Tree<T>], f: &mut Formatter) {
        write!(f.buf, "[");

        for x in v.iter() {
            write!(f.buf, " {}", *x);

        }

        write!(f.buf, " ]");
    }
}

