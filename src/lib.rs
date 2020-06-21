extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

// #[ ] の内側は「アトリビュート」と呼ばれ、次に来る文を何らかの形で修飾します。
// この場合、その文は外部で定義された関数を呼び出したいことを Rust に伝える extern です。
// アトリビュートは「wasm-bindgen はこれらの関数を見つける方法を知っている」ということを意味しています。
// 「alert 関数は s という名前の引数を一つ取る」ということを意味しています。

#[wasm_bindgen]
extern {
    pub fn alert(s: &str); // alertはJavascriptのalert関数
}

// JavaScript がこの Rust 関数を呼び出せるようにしてほしいということを意味します
// extern とは逆。自分が必要とする関数ではなく、外の世界に渡す関数

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
