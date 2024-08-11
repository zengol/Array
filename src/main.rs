fn main() {
    // construct mutable array
let mut array = [1, 2, 3];
println!("array = {:?}", array);
println!("array = {:#?}", array);

//access element array
println!("1er element in array: {}",
array[0]);
println!("2do element in array: {}",
array[1]);
println!("3er element in array: {}",
array[2]);


// mutate array element value
array[1] = 20;
println!(
    "2nd elemnt is now: {}",
    array[1]
);

//longitud del array
println!("array has {} elements",
array.len());

//sum array elements
let total: i32 = array.iter().sum();
println!("la suma total es: {}",total);

// sort array elements
// in descending index order
array.reverse();
println!(
    "descending index sort: {:?}",
    array
);

// sort array elements
// in ascendening value order
array.sort();
println!(
    "ascending value sort: {:?}",
    array
);

// construct array of arrays
let array_of_arrays = [
    [1, 2, 3],
    [4, 5, 6],
];
println!("array_of_array: {:#?}",
array_of_arrays

);

// access array_of_arrays elements
println!(
    "array 1, element 2: {}",
    array_of_arrays[0][1] 
);
println!(
    "array 2, element 3: {}",
    array_of_arrays[1][2]
);

}