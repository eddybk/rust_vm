use std::vec;

#[derive(Debug)]
#[derive(Clone, PartialEq)]
pub enum List <T: Clone>
{
    Mems(T, Box<List<T>>),
    Null
}


pub trait AsList {
    type T;
    fn as_list(&self) -> List<Self::T> where Self::T: Clone; 
}

impl<V> AsList for Vec<V> {
    type T = V;

    fn as_list(&self) -> List<Self::T> where Self::T: Clone {
        List::from(self.to_vec())
    }
}

pub trait Head {
    type Type;
    fn head(&self) -> Option<Self::Type>;
}
pub trait Tail {
    type Type;
    fn tail(&self) -> Option<List<Self::Type>> where Self::Type: Clone;
}

impl<T> Head for List<T> where T: Clone {
    type Type = T;

    fn head(&self) -> Option<Self::Type> {
        match &self {
            List::Mems(head, _tail) => Some(head.clone()),
            List::Null => None
        }
    }
}

impl<T> Tail for List<T> where T: Clone {
    type Type = T;

    fn tail(&self) -> Option<List<T>> {
        match &self {
            List::Mems(_head, tail) => Some(tail.as_ref().clone()),
            List::Null => None
        }
    }
}

impl<T> List<T> where T: Clone {
    pub fn from(vector: Vec<T>) -> List<T> {
        let mut vector = vector;
        let first = vector[0].clone();
        vector.reverse();
        vector.pop();
        let mut tail: List<T> = List::Null;
        for i in 0..vector.len() {
            tail = List::Mems(vector[i].clone(), Box::new(tail));
        }
        List::Mems(first, Box::new(tail))
    }

    pub fn at(&self, i: usize) -> T {
        self.as_vec()[i].clone()
    }
    pub fn last(&self) -> T {
        self.as_vec().last().unwrap().clone()
    }
    pub fn as_vec(&self) -> Vec<T> {
        let mut ret: Vec<T> = vec![];
        ret.push(match self.head() {
            Some(t) => t,
            None => panic!("Can not push Null into vec.")
        });
        let mut iter_obj = self.tail();
        'wh: while match iter_obj {
            Some(t) => {
                match t.head() {
                    Some(_v) => {
                        ret.push(match t.head() {
                            Some(v) => v,
                            None => panic!("Unreachable.")
                        });
                    },
                    None => break 'wh
                }; 
                iter_obj = t.tail();
                true
            },
            None => false
        }  { };

        ret
    }
}