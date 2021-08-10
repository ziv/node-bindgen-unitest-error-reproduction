use node_bindgen::derive::node_bindgen;

mod testing;

pub mod example {
    #[node_bindgen]
    pub fn sum(first: i32, second: i32) -> i32 {
        first + second
    }
}

