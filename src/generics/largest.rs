// Module: generics
// File: src/generics/largest.rs
//

/// This function takes a slice of type T that implements the PartialOrd trait and returns a reference to the largest element in the slice.
/// restricting type parameter `T` to types that implement the `PartialOrd` trait
/// tis is called Trait Bound
#[allow(dead_code)]
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest() {
        let numbers = vec![1, 2, 3, 4, 5];
        assert_eq!(*largest(&numbers), 5);

        let numbers = vec![10, 2, 3, 4, 5];
        assert_eq!(*largest(&numbers), 10);

        let numbers = vec![1, 2, 3, 4, 100];
        assert_eq!(largest(&numbers), &100);

        let char_list = vec!['y', 'm', 'a', 'q'];

        let result = largest(&char_list);
        println!("The largest char is {}", result);
        assert_eq!(result, &'y');

        let f = vec![1.1, 1.2];
        assert_eq!(*largest(&f), 1.2);

        // struct Point {
        //     x: i32,
        //     y: i32,
        // }
        // let p1 = Point { x: 1, y: 2 };
        // let p2 = Point { x: 3, y: 4 };
        // let p_list = vec![p1, p2];
        // assert_eq!(largest(&p_list).x, 3);
    }
}
