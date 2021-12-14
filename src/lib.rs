use proc_macro::{TokenStream, TokenTree};

fn get_env_name(item: TokenStream) -> String {
    match item.into_iter().collect::<Vec<_>>().remove(0) {
        TokenTree::Literal(s) => snailquote::unescape(&s.to_string()).unwrap(),
        tt => panic!("The first argument should be string literal, but {:}", tt),
    }
}

#[proc_macro]
pub fn code_env(item: TokenStream) -> TokenStream {
    let env_name: String = get_env_name(item);
    std::env::var(&env_name)
        .expect(&format!("env ${} not found", &env_name))
        .parse()
        .unwrap()
}
