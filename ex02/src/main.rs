use std::ops::AddAssign;
use num_traits::Num;
use std::ops::MulAssign;

//find out more about Num!
fn sum_vector<T: Num + AddAssign + Copy>(my_vec: &Vec<T>) -> T
{
    let mut check = 0;
    //protection: return error if length is 0
    let mut sum = my_vec[0];
    for item in my_vec.iter() {
        //see if there is another way to do this!
        if check == 0 {
            check = 1;
        }
        else {
            //overflow checks
            sum += *item;
        }
    }
    return sum;
}

 
fn multiply_all_by<T: Num + MulAssign + Copy> (my_vec: Vec<T>, my_val: T) {
    for item in my_vec.iter() 
    {
        *item *= my_val;
    }
}

fn cumulator<T: Num + AddAssign + Copy>(my_vec: Vec<T>) -> Vec<T> {
    let mut new_vec: Vec<T> = vec![];
    //protection: return error if length is 0
    let mut sum: T = my_vec[0];
    let mut check = 0;
    for item in my_vec {
        if check == 0 {
            check = 1;
        }
        else {
            //overflow checks
            sum += item;
        }
        new_vec.push(sum);
    }
    return new_vec;
}


//overflow checks?
fn main() {
    //pretty printing
    let vec= vec![0, 1, 2, 3, 4, 5];
    let total = sum_vector(&vec);
    let _new_vec: Vec<i32> = cumulator(vec);
    println!("Total is {}", total);

    //pretty printing
    let fvec= vec![0.4, 4.1, 2.2, 3.9098273, 4.23, 5.1];
    let total = sum_vector(&fvec);
    let _new_fvec: Vec<f64> = cumulator(fvec);
    println!("Total is {}", total);

    //pretty printing
}
