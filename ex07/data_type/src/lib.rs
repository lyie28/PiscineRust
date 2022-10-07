pub type DataVec = Vec<i32>;
pub type DataVecType = i32;

pub mod debug {
    use crate::prelude::*;

    pub fn debug_print(data: &DataVec) 
    {
        let _data_iter = data.iter();
        for val in _data_iter 
        {
            println!("{} ", val);
        }
    }
}

pub mod prelude {
    pub use crate::DataVec;
    pub use crate::DataVecType;
}

