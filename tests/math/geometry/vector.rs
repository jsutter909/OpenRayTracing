
#[cfg(test)]
mod tests {

    #[test]
    fn test_vector_construction() {
        let _vec1 = Vector::new([1.0, 2.0]);
        let _vec2 = Vector::new([3.0, 4.0]);
    }

    #[test]
    fn test_vector_indexing() {
        let vec1 = Vector::new([1.0, 2.0]);
        let vec2 = Vector::new([3.0, 4.0]);

        assert_eq!(vec1[0], 1.0);
        assert_eq!(vec1[1], 2.0);
        assert_eq!(vec2[0], 3.0);
        assert_eq!(vec2[1], 4.0);
    }

    #[test]
    fn test_dot_product() {
        let vec1 = Vector::new([1.0, 2.0]);
        let vec2 = Vector::new([3.0, 4.0]);

        assert_eq!(dot(&vec1, &vec2), 11.0);
        assert_eq!(dot(&vec2, &vec1), 11.0);
    }
}

