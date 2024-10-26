// 导入此包定义的trait时自动同时导入hello_macro_derive包定义的宏
pub use hello_macro_derive::HelloMacro;
pub use hello_macro_derive::route;
pub trait HelloMacro {
    fn hello_macro() -> ();
}
