extern crate shape;

#[cfg(test)]
mod tests {
    use shape::Rectangle;
    use shape::Circle;

    #[test]
    fn should_count_area_of_rectangle() {
        let rect = Rectangle { width: 50, height: 50 };

        assert_eq!(rect.area(), 2500);
    }

    #[test]
    fn should_count_area_of_circle() {
        let circle = Circle { diameter: 14.0 };
        assert_eq!(circle.area(), 153.93805);
    }
}

