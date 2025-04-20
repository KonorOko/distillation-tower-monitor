#[cfg(test)]
mod tests {
    use crate::calculations::service::CalculationService;

    #[test]
    fn test_calculate_composition() {
        let service = CalculationService::new();
        let result = service
            .calculate_composition(Some(0.5), 80.0, Some(1e-6), Some(100))
            .unwrap();

        assert!(result.x_1.unwrap() > 0.0 && result.x_1.unwrap() < 1.0);
        assert!(result.y_1.unwrap() > 0.0 && result.y_1.unwrap() < 1.0);
    }

    #[test]
    fn test_interpolate_temps() {
        let service = CalculationService::new();

        let temps = service.interpolate_temps(5, 70.0, 90.0);
        assert_eq!(temps.len(), 5);
        assert!(temps[0] == 70.0);
        assert!(temps[1] == 75.0);
        assert!(temps[2] == 80.0);
        assert!(temps[3] == 85.0);
        assert!(temps[4] == 90.0);
    }
}
