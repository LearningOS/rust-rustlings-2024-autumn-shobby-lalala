// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.

use std::mem;

fn main() {
    let my_option: Option<()> = None;
    if my_option.is_none() {
        // 这里可以不需要做任何操作
    }

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6,
    ];
    println!("My array! Here it is: {:?}", my_arr);

    let my_empty_vec: Vec<i32> = vec![]; // 创建一个空的 Vec
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // 让我们交换这两个值！
    mem::swap(&mut value_a, &mut value_b); // 使用 std::mem::swap
    println!("value a: {}; value b: {}", value_a, value_b);
}
