use crate::vector::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec2_operations() {
        let mut vec1 = Vec2 { x: 2.0, y: 3.0 };
        let vec2 = Vec2 { x: 1.0, y: 2.0 };

        // Addition
        vec1 += vec2;
        assert_eq!(vec1.x, 3.0);
        assert_eq!(vec1.y, 5.0);

        // Subtraction
        vec1 -= vec2;
        assert_eq!(vec1.x, 2.0);
        assert_eq!(vec1.y, 3.0);

        // Multiplication
        vec1 *= vec2;
        assert_eq!(vec1.x, 2.0);
        assert_eq!(vec1.y, 6.0);

        // Division
        vec1 /= vec2;
        assert_eq!(vec1.x, 2.0);
        assert_eq!(vec1.y, 3.0);

        // Modulo
        vec1 %= vec2;
        assert_eq!(vec1.x, 0.0);
        assert_eq!(vec1.y, 1.0);
    }
}
