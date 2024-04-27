fn main() {
    println!("Hello, world from ICP!");

    // integer => i ve u ile başlar. i işaretli olur
    // 8, 16, 24, 32, 128 bit. En az - 2 * 7 ve 2 * 7 - 1
    // işaretsiz olan 0 ve 2 ** 8 - 1
    // arch -Z isize, usize
    let mut i8_var = 8; // Değişkeni mutable yapın

    let u32_var = 23;
    let u64_var = 23;
    let u_total = u32_var as u64 + u64_var as u64; // u_total'ı u64'e dönüştürün

    let i8_var2: u128 = 239033;
    i8_var += 10; // bunu yapmak daha masraflı
    println!("{}", i8_var);

    let seperated_var = 123_456_789;
    println!("{}", seperated_var);

    println!("{1} - {0}", i8_var, i8_var2);

    let i_f_var = -34354.5;
    println!("{}", i_f_var);

    let mut char_var = 'a';

    let mut octal_var: u16 = 0o51351;
    let hex_var: u32 = 0xA09;
    let binary_var: u32 = 0b10110;

    //array tuple
    let arr_var = [1, 2, 3];
    let arr_var2 = [1; 3];
    let arr_var3: [u32; 4] = [1, 2, 3, 4];

    let tuple_var = (2, 3.7, 'w');

    //string slices
    let str_var: &str = "ISP Hub";

    //strings
    let string_var = String::new();
    let string_var2 = String::from("Türkiye Rust Community");

    println!("{}", string_var2);

    //vectors
    //pointers, capacity, length
    let vec_arr = vec![1,2,3,4];
    let vec_arr2: Vec<u32>  = vec![1,2,3,4];

    println!("{:?}", vec_arr);

    #[derive(Debug, PartialEq)]
    struct StructName{
        field1: i32,
        field2: f64
    }

    let struct_var = StructName {
        field1: 1,
        field2: 1.240,
    };

    println!("{:?}", struct_var);

    #[derive(Debug, PartialEq)]
    enum Department {
        Computer(
            Vec<String>),
        Maths((u16, String))
    }

    #[derive(Debug, PartialEq)]
    struct Student{
        field1: String,
        department: Department,
        field2: i32,
        field3: f64
    }

    let mut student = Student {
        field1: "Yaşar Güzel".to_string(),
        department: Department::Computer(vec!("Rust".to_string())),
        field2: 2,
        field3: 99.9,
    };

    student.department = Department::Maths((10,"Rust".to_string()));

    println!("{:?}", student);

    enum EnumName {
        Enum1,
        Enum2(StructName),
        Enum3(u32),
        Enum4((i32,f64,char))
    }

    // control  statements
    // if, if - else, 
    if student.department == Department::Computer(vec!["ICP".to_string(), "Rust".to_string()]){
        println!("IC Hub");
    }else {
        println!("Rust");
    }


    // loops
    loop{
        println!("One time execution");
        break;
    }

    let mut index = 0;
    while index < 2 {  
        println!("{}", index);
        index += 1;
    }

    for i in (1..=5).step_by(2){
        println!("for loop: {}", i);
    }

    match student.department{
        Department::Computer(lessons) => {
        println!("Alınan Dersler: {:?}", lessons);
        }
        _ => {
            println!("Daha sonra dene");
        }
    }
     
}