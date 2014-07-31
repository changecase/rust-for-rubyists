fn main() {
    let mut a_vector = vec!(4i);
    let mut mut_vector = a_vector;
    *mut_vector.get_mut(0) = 5;

    println!("The first number is {:d}.", *mut_vector.get(0));
}
