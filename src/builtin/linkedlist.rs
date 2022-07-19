// singly linked list
use bincode::{Encode, Decode};
use std::rc::Rc;
use std::fmt::{Debug, Formatter};

// from cource.rs
#[derive (Clone, Encode, Decode, PartialEq)]
pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

#[derive (Encode, Decode, PartialEq)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T: std::fmt::Debug> Debug for List<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fn rec_fmt<F: std::fmt::Debug>(list: &List<F>, f: &mut Formatter<'_>) -> std::fmt::Result {
            match list.head() {
                None => write!(f, "end"),
                Some(x) => {
                    write!(f, "{:?}, ", x)?;
                    rec_fmt(&list.tail(), f)
                }
            }
        }
        write!(f, "[")?;
        rec_fmt(self, f)?;
        write!(f, "]")
    }

}
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        // take: Takes the value out of the option, leaving a None in its place.
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn prepend(&self, elem: T) -> List<T> {
        List { head: Some(Rc::new(Node {
            elem: elem,
            next: self.head.clone(),
        }))}
    }

    pub fn tail(&self) -> List<T> {
        // and_then: like map, but for Option
        List { head: self.head.as_ref().and_then(|node| node.next.clone()) }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn list_basics() {
        let list = List::new();
        assert_eq!(list.head(), None);

        let list = list.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), None);

        // Make sure empty tail works
        let list = list.tail();
        assert_eq!(list.head(), None);

    }
}


