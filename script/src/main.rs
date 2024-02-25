// fn append_str(str: &mut String, n: i32) {
//     for i in 0..n {
//         let nstr = str.clone() + &i.to_string();

//         str = nstr
//     }
// }

// fn main() {
//     let mut str = String::new();
//     append_str(&mut str, 5);
//     println!("{:?}", str); // This will print: [5, 4, 3, 2, 1]
// }

struct Person {
    id: i32,
}

fn generate_id(p: &mut Person, n: i32) {
    for i in 0..n {
        // Directly append the number to the string
        p.id = p.id + n
    }
}

fn main() {
    let mut p = Person { id: 0 };
    generate_id(&mut p, 5);
    println!("{:?}", p); // This will print: "01234"
}
