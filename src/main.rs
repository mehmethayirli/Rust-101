fn main() {
    let i8_var_i8: u128 = 123;
    println!("Hello, {}!", i8_var_i8);

    let u32_var: u32 = 23;
    let u64_var: u64 = 23;
    // let result = u32_var + u64_var;
    // let result = u64_var + u32_var;
    // let result = (u32_var + u64_var) as u128;
    // let result = (u64_var + u32_var) as u128;
    // let result: u64 = u32_var + u64_var;
    let result = u32_var as u64 + u64_var;

    println!("{2}, {1}, {0}, {2}!", 12, 34, '🦀');

    let foo = -3.1415;

    println!("0.1 + 0.2 = {}", 0.1 + 0.2);
    // 0.1 + 0.2 = 0.30000000000000004

    let mut char_var = 'a';
    // char_var += 2;

    let one = [1, 2, 3];
    let two = [2; 3];
    // let three: [u32; 6] = [2; 3];
    // println!("{}, {}, {}", one, two, three);
    // println!("{}, {}", one, two);
    println!("{:?}, {:?}", one, two);

    let tuple = (2, 3.7, 'd');
    let string_slice = "ICP Hub";
    let string = String::new();
    let string2 = String::from("Türkiye Rust Community");
    println!("{}", string2);

    // Vectors: pointers, capacity, length
    let vec_arr = vec![1, 2, 3, 4];
    // println!("{}", vec_arr);
    println!("{:?}", vec_arr);

    #[derive(Debug)]
    struct StructName {
        field1: i32,
        field2: f64,
    }

    let struct_var = StructName {
        field1: 42,
        field2: 5.757,
    };

    // println!("{}", struct_var);
    println!("{:?}", struct_var);

    // #[derive(Debug)]
    // struct Student {
    //     // name: &str,
    //     name: String,
    //     surname: String,
    //     department: String,
    //     score: u32,
    // }
    // let mut student1 = Student {
    //     name: "Osman Nuri".to_string(),
    //     surname: String::from("Yıldız"),
    //     department: "Computer Engineering".to_string(),
    //     score: 100,
    // };
    // student1.department = "Rustacean".to_string();
    // println!("{:?}", student1);

    enum EnumName {
        Variant1,
        Variant2(StructName),
        Variant3(u32),
        Variant4((i32, f64, char)),
        Variant5(),
    }

    #[derive(Debug, PartialEq)]
    enum Department {
        ComputerEngineering(Vec<String>),
        ElectricalEngineering((u16, String)),
        MachineEngineering((u16, String)),
        Science,
        Education,
        Social,
    }
    #[derive(Debug)]
    struct Student {
        name: String,
        surname: String,
        department: Department,
        score: u32,
    }
    let mut student2 = Student {
        name: "Osman Nuri".to_string(),
        surname: String::from("Yıldız"),
        department: Department::ComputerEngineering(vec!["Rust".to_string()]),
        score: 100,
    };
    student2.department = Department::ElectricalEngineering((10, "Rust".to_string()));
    println!("{:?}", student2);

    // if student2.department == Department::ComputerEngineering(vec!["ICP".to_string()]) {
    if student2.department
        == Department::ComputerEngineering(vec!["ICP".to_string(), "Rust".to_string()])
    {
        println!("True!")
    } else {
        println!("False!")
    }

    loop {
        println!("loop");
        break;
    }

    let mut i = 0;
    while i < 3 {
        println!("while loop: {}", i);
        i += 1;
    }

    // for j in 1..5 {
    // for j in 1..=5 {
    for j in (1..=5).step_by(2) {
        println!("for loop: {}", j)
    }

    match student2.department {
        // Department::ComputerEngineering(_) => todo!(),
        Department::ComputerEngineering(lessons) => {
            println!("Alınan dersler: {:?}", lessons)
        }
        // Department::ElectricalEngineering(_) => todo!(),
        // Department::MachineEngineering(_) => todo!(),
        // Department::Science => todo!(),
        // Department::Education => todo!(),
        // Department::Social => todo!(),
        _ => {
            println!("Daha sonra dene")
        }
    }

    #[derive(Debug)]
    enum MembershipType {
        Economy,
        Business,
        VIP,
    }
    #[derive(Debug)]
    struct Member {
        name: String,
        surname: String,
        membership_type: MembershipType,
    }
    #[derive(Debug)]
    struct Library {
        name: String,
        members: Vec<Member>,
    }
    let lib = Library {
        name: "Mamak Halk Kütüphanesi".to_string(),
        members: vec![
            Member {
                name: "Veli".to_string(),
                surname: "Uysal".to_string(),
                membership_type: MembershipType::VIP,
            },
            Member {
                name: "Mehmet".to_string(),
                surname: "Hayırlı".to_string(),
                membership_type: MembershipType::Business,
            },
        ],
    };
    println!("{:#?}", lib);

    let mut return_var = function_print("Naber la");
    return_var.push_str(" geldin mi?");
    println!("{}", return_var);
    println!("-----");

    // Copy (fixed size)
    let fs_var1 = 42;
    let mut fs_var2 = fs_var1;
    fs_var2 = 123;
    println!("fs_var1: {}", fs_var1);
    println!("fs_var2: {}", fs_var2);
    println!("-----");

    // Move (non-fixed size)
    let nfs_var1 = "Değişken".to_string();
    let mut nfs_var2 = nfs_var1;
    // At this moment, nfs_var1 has lost the ownership, so we can no longer use it
    // println!("nfs_var1: {}", nfs_var1); // ERROR
    println!("nfs_var2: {}", nfs_var2);
    println!("-----");

    // Clone (non-fixed size)
    let mut nfs_var3 = nfs_var2.clone();
    nfs_var3.push_str(" Değişir");
    println!("nfs_var2: {}", nfs_var2);
    println!("nfs_var3: {}", nfs_var3);
    println!("-----");

    // Immutable borrowing/shared reference (1:N)
    let borrowing1 = &nfs_var2;
    println!("nfs_var2: {}", nfs_var2);
    println!("borrowing1: {}", borrowing1);
    println!("-----");

    // Mutable borrowing (1:1)
    let borrowing2 = &mut nfs_var2;
    // println!("nfs_var2: {}", nfs_var2); // This takes the reference back and ERROR
    println!("borrowing2: {}", borrowing2);
    // println!("nfs_var2: {}", nfs_var2); // This takes the reference back and ERROR
    println!("borrowing2: {}", borrowing2);
    println!("nfs_var2: {}", nfs_var2); // This takes the reference back
    println!("-----");

    // Traits
    // OOP -> inject method
    // Struct yapısını daha class gibi kullanmamıza olanak sağlar
    // Idioms ~= best practices

    trait DenemeTrait {
        fn default_method(&self) {
            println!("== Default method ==");
        }

        fn print_all(&self);
    }

    impl DenemeTrait for StructName {
        fn print_all(&self) {
            println!("== field 1: {}, field 2: {} ==", self.field1, self.field2);
        }

        fn default_method(&self) {
            println!("== Default method override ==");
        }
    }

    struct_var.print_all();
    struct_var.default_method();

    impl Student {
        fn print_all(&self) {
            println!("{} {}, notu {}.", self.name, self.surname, self.score);
        }
    }
    let student3 = Student {
        name: "Mehmet".to_string(),
        surname: String::from("Hayırlı"),
        department: Department::ComputerEngineering(vec!["Unity".to_string()]),
        score: 100,
    };
    student3.print_all();

    //threads concurrency parallelism channel

    let thread1 = thread::spawn( || {
       for i in 1..=10{
           println!("Thread : {}",i);
       }
    });

    thread1.join().unwrap();
    println!("After thread");

}

fn function_print(s: &str) -> String {
    println!("Gelen ifade: {}", s);
    // return String::from(s);
    String::from(s)
}