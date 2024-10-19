// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
// hint.

mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }

    // 为了能在外部使用宏，需要将它引入到作用域中
    pub(crate) use my_macro; // 让宏在当前模块可见
}

fn main() {
    macros::my_macro!(); // 使用模块路径调用宏
}
