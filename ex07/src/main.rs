use core::execute as execute_core;
use onnx::execute as execute_onnx;
use data_type::debug;
use linalg::treatment;

fn main() {
    let test = vec![1, 2, 3, 4];

    println!("Testing onnx!");
    execute_onnx::launch_model(&test);

    println!("Testing linalg && data_type!");
    let new_vec = treatment::multiply(&test, 10);
    println!("New vec from linalg::treatment::multiply * 10");
    debug::debug_print(&new_vec);
    let new_vec2 = treatment::divide(&new_vec, 10);
    println!("New vec from linalg::treatment::divide / 10");
    debug::debug_print(&new_vec2);

    let new_vec3 = treatment::full_conversion_onnx(&new_vec2);
    println!("New vec from linalg::treatment::full_conversion_onnx");
    debug::debug_print(&new_vec3);

    println!("Testing core!");
    execute_core::handle_vector(&new_vec3, 0);
    execute_core::handle_vector(&new_vec3, 1);
}
