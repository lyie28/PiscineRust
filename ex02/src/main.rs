fn sum_vector(my_vec: &Vec<i32>) -> i32
{
    let mut check = 0;
    if my_vec.len() == 0 {
        return check;
    }

    let mut sum = my_vec[0];
    for item in my_vec.iter() {
        if check == 0 {
            check = 1;
        }
        else {
            if sum + *item > i32::MAX {
                println!("Erorr: value reached INT_MAX.");
                return i32::MAX;
            }
            if sum + *item < i32::MIN {
                println!("Erorr: value reached INT_MIN.");
                return i32::MIN;
            }
            sum += *item;
        }
    }
    return sum;
}

 
fn multiply_all_by(my_vec: &mut Vec<i32>, my_val: i32) {
    for item in my_vec.iter_mut() 
    {
        if *item * my_val > i32::MAX {
            println!("Erorr: value reached INT_MAX.");
            *item = i32::MAX;
        }
        if *item * my_val < i32::MIN {
            println!("Erorr: value reached INT_MIN.");
            *item = i32::MIN;
        }
        *item *= my_val;
    }
}

fn cumulator(my_vec: &Vec<i32>) -> Vec<i32> {
    let mut new_vec: Vec<i32> = vec![];
    if my_vec.len() == 0 {
        return new_vec;
    }
    let mut sum: i32 = my_vec[0];

    let mut check = 0;
    for item in my_vec {
        if check == 0 {
            check = 1;
        }
        else {
            if sum + *item > i32::MAX {
                println!("Erorr: value reached INT_MAX.");
                sum = i32::MAX;
            }
            if sum + *item < i32::MIN {
                println!("Erorr: value reached INT_MIN.");
                sum = i32::MIN;
            }
            sum += item;
        }
        new_vec.push(sum);
    }
    return new_vec;
}

fn print_vector(vec: &Vec<i32>) {
    print!("Vector contains: ");
    for item in vec {
        print!("[{}] ", item);
    }
    println!("");
}
fn main() {

    let mut vec= vec![0, 1, 2, 3, 4, 5];
    print_vector(&vec);
    let mut total = sum_vector(&vec);
    println!("Total sum of vec is: {}", total);

    
    println!("Multiplying vector by 10: {}", total);
    multiply_all_by(&mut vec, 10);
    total = sum_vector(&vec);
    println!("Total sum of vec is: {}", total);
    print_vector(&vec);

    let new_vec: Vec<i32> = cumulator(&vec);
    total = sum_vector(&vec);
    println!("Total sum of vec is: {}", total);
    total = sum_vector(&new_vec);
    println!("Total sum of new vec is: {}", total);
    print_vector(&vec);
    print_vector(&new_vec);
}
