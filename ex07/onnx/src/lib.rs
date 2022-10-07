pub mod execute {
    use crate::prelude::*;

    pub fn launch_model(data: &DataVec)
    {
        handle_vector(data, 1);
    }
}

pub mod prelude {
    pub use core::prelude::*;
}
