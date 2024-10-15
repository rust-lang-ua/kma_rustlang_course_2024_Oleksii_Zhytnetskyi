#[derive(Debug)]
struct MyStruct(i32, i32, i32);

fn main() {
    let my_struct = MyStruct(2023, 10, 11);
    println!("my_struct : {:?} @ {:p}", my_struct, &my_struct);
}

