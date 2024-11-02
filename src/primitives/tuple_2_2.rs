use std::fmt;
use std::fmt::Formatter;

pub fn reverse(pair:(i32, bool)) ->(bool, i32){
    let ( inter, boolean) = pair;
    (boolean, inter)
}

pub fn transpose(matrix:Matrix) -> Matrix{
    let Matrix(a,b,c,d) =matrix;
    Matrix(a,c,b,d)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);
impl fmt::Display for Matrix  {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})\n({},{})",self.0,self.1,self.2,self.3)
    }
    
}

pub(crate) fn test_main() {
    let long_tuple=(1u8, 2u16, 3u32, 4u64,-1i8,-2i16,-3i32,-4i64,0.1f32,0.2f64,'a',true);

    // 通过下标访问具体的值
    println!("long tuple first value:{}", long_tuple.0);
    println!("long tuple second value:{}", long_tuple.1);

    // 元组来充当元组的元素
    let tuple_of_tuples = ((1u8,2u16,2u32), (4u64,-1i8), -2i16);

    // 元组可以打印
    println!("tuple of tuples:{:?}", tuple_of_tuples);

    // let too_long_tuple = (1,200,88,90,80,3,4,5,67,1,4,1,4);
    // print!("too long tuple{:?}",too_long_tuple);
    let pair = (1, true);
    print!("pair is {:?}", pair);

    print!("the reversed pair is {:?}\n", reverse(pair));

    print!("on element tuple:{:?}\n", (5u32,));
    print!("just an integer: {:?}\n",(5u32));
//元组可以被解构
    let tuple = (1,"hello",4.5,true);
    let (a,b,c,d)=tuple;
    println!("{:?},{:?},{:?},{:?}",a,b,c,d);

    let matrix = Matrix(1.1,1.2,2.1,2.2);
    print!("{}", matrix);
    let transposed = transpose(matrix);
    print!("\n{}", transposed);


}