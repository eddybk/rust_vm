pub mod io {
    pub struct Out;
    impl<T: std::fmt::Display> std::ops::BitOr<T> for Out {
        type Output = Out;

        fn bitor(self, rhs: T) -> Self::Output {
            print!("{}", rhs);
            Self::Output {}
        }
    } 
    impl<T: std::fmt::Debug> std::ops::Rem<T> for Out {
        type Output = Out;

        fn rem(self, rhs: T) -> Self::Output {
            print!("{:#?}", rhs);
            Self::Output {}
        }
    }
    impl Out {
        pub fn print <T: std::fmt::Display> (thing: T) -> Out {
            print!("{}", thing);
            Out {}
        }
    }
}
macro_rules! cout {
    
    [$x:expr] => {
        crate::io::io::Out::print($x)
    };
    [] => {
        crate::io::io::Out::print("")
    }
}

pub(crate) use cout;