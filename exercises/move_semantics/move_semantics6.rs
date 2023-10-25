// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let data = "Rust is great!".to_string();    // String对象

    get_char(&data);    // 传递的是不可变引用

    string_uppercase(data); // 传递所有权
}

// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    let data1 = data.to_uppercase();

    println!("{}", data1);  // data1销毁
}
