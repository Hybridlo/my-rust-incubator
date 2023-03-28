use btreemap_macro::btreemap_procedural;

macro_rules! btreemap_declarative {
    ( $($k:tt: $v: expr),* $(,)? ) => {
        {
            let mut temp_map = std::collections::BTreeMap::new();
            $(
                temp_map.insert($k, $v);
            )*
            temp_map
        }
    };
}

fn main() {
    println!("Implement me!");
}

#[cfg(test)]
mod tests {
    use btreemap_macro::btreemap_procedural;

    #[test]
    fn test_btreemap_macro_declarative() {
        let map = btreemap_declarative!(
            "hi": "hello",
            "bye": "goodbye"
        );

        assert!(map.contains_key("hi"));
        assert!(matches!(map.get("bye"), Some(&"goodbye")));

        // will not compile - mismatched types
        /* let map = btreemap_declarative!(
            3: "hello",
            "bye": "goodbye"
        ); */
    }

    #[test]
    fn test_btreemap_macro_procedural() {
        let map = btreemap_procedural!(
            "hi": "hello",
            "bye": "goodbye"
        );

        assert!(map.contains_key("hi"));
        assert!(matches!(map.get("bye"), Some(&"goodbye")));

        // will not compile - mismatched types
        /* let map = btreemap_procedural!(
            3: "hello",
            "bye": "goodbye"
        ); */
    }
}