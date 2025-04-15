fn process_vector_with_map<F>(vec: Vec<i32>, f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    vec.into_iter().map(f).collect()
}

// Implementation using a for loop
fn process_vector_with_for_loop<F>(vec: Vec<i32>, f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    let mut result = Vec::new();
    for x in vec {
        result.push(f(x)); 
    }
    result
}

fn main() {
    let numbers = vec![1, 2, 3];

    // Using process_vector_with_map
    let doubled_map = process_vector_with_map(numbers.clone(), |x| x * 2); 
    let replaced_map = process_vector_with_map(numbers.clone(), |x| if x > 2 { 0 } else { x }); 

    println!("Using map and collect:");
    println!("Doubled: {:?}", doubled_map);
    println!("Replaced: {:?}", replaced_map);

    // Using process_vector_with_for_loop
    let doubled_loop = process_vector_with_for_loop(numbers.clone(), |x| x * 2); 
    let replaced_loop = process_vector_with_for_loop(numbers, |x| if x > 2 { 0 } else { x }); 

    println!("Using for loop:");
    println!("Doubled: {:?}", doubled_loop);
    println!("Replaced: {:?}", replaced_loop);
}