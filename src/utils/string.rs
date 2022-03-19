mod list;

use list::List;
use list::{Head, Tail};

use crate::list::AsList;

#[derive(Debug, PartialEq)]
pub struct StringView {
    data: List<char>,
    size: usize
}

impl StringView {
    pub fn new() -> StringView {
        StringView { data: List::Null, size: 0 }
    }
    pub fn from(data: String) -> StringView {
        StringView {
            data: List::from(data.chars().collect()),
            size: data.len()
        }
    }
    pub fn size(&self) -> usize {
        self.size
    }
}


impl Head for StringView {
    type Type = char;
    fn head(&self) -> Option<Self::Type> {
        self.data.head()
    }
}

impl Tail for StringView {
    type Type = char;
    fn tail(&self) -> Option<List<<Self as Tail>::Type>> where <Self as Tail>::Type: Clone { 
        self.data.tail()
    }
}

impl AsList for String {
    type T = char;
    
    fn as_list(&self) -> crate::list::List<Self::T> where Self::T: Clone {
        self.chars().collect::<Vec<_>>().as_list()
    }
}