fn variable_type<T>(_var: &T) {
    //check if can crash in any way
    println!("{}", std::any::type_name::<T>());
}

fn main() {
    //TODO: check this works with absolutely all variable types. Find list of them and check with all
    variable_type(&"Laura");
    variable_type(&42);
}