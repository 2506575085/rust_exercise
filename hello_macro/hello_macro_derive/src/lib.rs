use proc_macro::TokenStream;
use quote::quote;

// derive宏，用于给 struct 或 enum 添加 trait 实现
#[proc_macro_derive(HelloMacro)]
// 定义宏通用模版
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_hello_macro(&ast)
}

// 具体实现
fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}

// TODO 有问题, 类属性宏
#[proc_macro_attribute]
pub fn route(attr: TokenStream, _item: TokenStream) -> TokenStream {
    let ast = syn::parse(attr).unwrap();
    impl_route_macro(&ast)
}
fn impl_route_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        println!("Hello, macro! My name is {}!", stringify!(#name));
    };
    gen.into()
}