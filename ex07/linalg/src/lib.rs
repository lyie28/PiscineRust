pub mod treatment {
    use data_type::prelude::*;

    pub fn multiply(data: &DataVec, multiplier: DataVecType) -> DataVec
    {
        let data_opt: DataVec = data.iter().map(|x| x * multiplier).collect();
        data_opt
    }

    pub fn divide(data: &DataVec, divider: DataVecType) -> DataVec
    {
        let data_opt: DataVec = data.iter().map(|x| x / divider).collect();
        data_opt
    }

    pub fn full_conversion_onnx(data: &DataVec) -> DataVec
    {
        let mut data_opt: DataVec = multiply(data, 64);
        data_opt = divide(&data_opt, 2);
        data_opt
    }
}

pub mod prelude {
    #[allow(unused_imports)]
    pub use data_type::prelude::*;
    #[allow(unused_imports)]
    pub use crate::treatment::*; 
}