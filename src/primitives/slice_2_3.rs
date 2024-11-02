use std::mem;

pub fn analyze_slice(slice: &[i32]){
    println!("\nfirst element of slice:{}", slice[0]);
    print!("\nthe slice has {} elements", slice.len());
}

pub fn slice_main(){
    let xs:[i32;5] = [1,3,4,5,6];
    let ys:[i32;500] = [0;500];

    print!("\nfirst element of the array:{}",xs[0]);

    print!("array occupies {} bytes\n", mem::size_of_val(&xs));

    analyze_slice(&xs);
    analyze_slice(&ys[1..4]);

    println!("\n{}",xs[4])
}