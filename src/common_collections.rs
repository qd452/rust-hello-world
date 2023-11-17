#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn test_vector_ops() {
        let v = vec![1, 2, 3, 4, 5];

        assert_eq!(v[0], 1);
        assert_eq!(v[0..1], [1]);

        let v_e: Vec<i32> = Vec::new();
        assert_eq!(v_e.len(), 0);

        let v3 = &v[3];
        println!("v3: {v3}");
        assert_eq!(v3, &4);

        let v3 = v.get(3);
        assert_eq!(v3, Some(&4));

        let v100 = v.get(100);
        assert_eq!(v100, None);

        for i in &v {
            assert!(*i > 0);
        }

        let mut v = vec![1, 2, 3, 4, 5];
        for i in &mut v {
            *i *= 2;
        }
        assert_eq!(v, [2, 4, 6, 8, 10]);

        {
            let v = vec![1, 2, 3, 4, 5];
            println!("v: {:?}", v);
        }
        // println!("v: {:?}", v); // error[E0425]: cannot find value `v` in this scope
        println!("v: {:?}", v);
        assert_eq!(v, [2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_hash_map_ops() {
        let mut hm: HashMap<String, u64> = HashMap::new();
        assert_eq!(hm.len(), 0);
        hm.insert("Blue".to_string(), 1);
        assert_eq!(hm["Blue"], 1);
        assert_eq!(hm.len(), 1);
        hm.insert("Red".to_string(), 2);
        let blue = "Blue".to_string();
        let bv = hm.get(&blue).copied().unwrap_or(0);
        assert_eq!(bv, 1);
        hm.insert("Blue".to_string(), 3);
        assert_eq!(hm["Blue"], 3);
        assert_eq!(bv, 1);

        for (k, v) in &hm {
            println!("{}: {}", k, v);
            if k == "Blue" {
                assert_eq!(v, &3);
            } else if k == "Red" {
                assert_eq!(v, &2);
            }
        }

        // Adding a Key and Value Only If a Key Isn’t Present
        hm.entry("Yellow".to_string()).or_insert(4);
        assert_eq!(hm["Yellow"], 4);
        hm.entry(String::from("Blue")).or_insert(5);
        assert_eq!(hm["Blue"], 3);

        // Updating a Value Based on the Old Value (eg can be used to count words)
        let text = "hello world wonderful world";
        let mut hm: HashMap<String, u64> = HashMap::new();
        for word in text.split_whitespace() {
            let count = hm.entry(word.to_string()).or_insert(0);
            *count += 1;
        }

        assert_eq!(hm["hello"], 1);
        assert_eq!(hm["world"], 2);
        assert_eq!(hm["wonderful"], 1);
    }
}
