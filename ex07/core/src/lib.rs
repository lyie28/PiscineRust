pub mod execute {
    use crate::prelude::*;

    pub fn handle_vector(data: &DataVec, type_model: u8)
    {
        let new_vec: DataVec;
        if type_model == 0
        {
            new_vec = multiply(data, 16);
        }
        else if type_model == 1
        {
            new_vec = full_conversion_onnx(data);
        }
        else
        {
            panic!("Unknown format, abording");
        }
        print(&new_vec);
    }

    fn print(data: &DataVec) 
    {
        let _data_iter = data.iter();
        for val in _data_iter 
        {
            println!("{} ", val);
        }
    }
}

pub mod prelude {
    pub use linalg::prelude::*;
    pub use crate::execute::*; 
}
