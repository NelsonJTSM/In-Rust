pub fn difficulty_rating() -> f32{
    1.5
}

pub fn hours_spent() -> f32 {
    7.0
}

mod tests {
    #[allow(unused)]
    use crate::*;

    // difficulty_rating() must return a double on the range [1.0, 5.0].
    #[test]
    fn difficulty_rating_correct_range() {
        let rating = difficulty_rating();
        assert!(rating >= 1.0 && rating <= 5.0, "difficulty_rating() must return a double on the range [1.0, 5.0], instead was: {}", rating);
    }

    // hours_spent() must be strictly greater than zero.
    #[test]
    fn hours_spent_greater_than_one() {
        let hours = hours_spent();
        assert!(hours >= 0.0, "hours_spent() must be strictly greater than zero, instead was: {}", hours);
    }
}
