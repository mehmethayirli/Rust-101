fn main() {
    println!("Hello, world from ICP!");

    // integer => i ve u ile başlar. i işaretli olur
    // 8, 16, 24, 32, 128 bit. En az - 2 * 7 ve 2 * 7 - 1
    // işaretsiz olan 0 ve 2 ** 8 - 1 
    // arch -Z isize, usize
    let i8_var = 8;

    let u32_var = 23;
    let u64_var:i32  = 23;
    let u_total = u32_var as u64 + u64_var as u64; // u_total'ı u64'e dönüştürün

    let i8_var2: u128 = 239033;
    //i8_var = i8_var + 10;
    //i8_var += 10; //bunu yapmak daha masraflı

    println!("{}", i8_var);

    let seperated_var:u32 = 123_456_789;
    println!("{}", seperated_var);

    println!("{1} - {0}", i8_var, i8_var2);

    let i_f_var = -12121.2;
    println!("{}",i_f_var);

    //f -> f32 f64
    let mut char_var = 'a';

    //octal, hexadecimal, binary literals
    let octal_var:u16 = 0o72234;
    let hex_var:u32 = 0xAF09;
    let binary_var = 0b101010;

    //array ve tuple
    let arr_var = [1,2,3];
    let arr_var2 = [1;3];
    let arr_var3: [u32; 4] = [1,3,4,6];

    let tupple_var = (2,3,7,'d');

    //string slices
    let str_var = "ICP Hub";

    //string
    let string_var = String::new();
}