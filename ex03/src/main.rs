fn variable_type<T>(_var: &T) {
    println!("{}", std::any::type_name::<T>());
}
struct Test {
    _name: String,
    _number: u64,
}
fn main() {
    let test = Test {
        _name: String::from("Laura"),
        _number: 29138918273,
    };

    let hello = vec![5.5, 6.6];
    variable_type(&"Laura");
    variable_type(&42);
    variable_type(&test);
    variable_type(&hello);

}