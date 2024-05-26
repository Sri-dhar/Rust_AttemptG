use std::collections::HashMap;

// Make car_prices public
pub struct Cars {
    pub car_prices: HashMap<String, u32>,
}

impl Cars {
    // Create a new Cars struct
    pub fn new() -> Cars {
        let mut car_prices: HashMap<String, u32> = HashMap::new();

        // Add some random cars and their prices to the map
        car_prices.insert(String::from("Toyota Camry"), 25000);
        car_prices.insert(String::from("Honda Civic"), 22000);
        car_prices.insert(String::from("Ford Mustang"), 40000);
        car_prices.insert(String::from("Chevrolet Corvette"), 60000);

        Cars { car_prices }
    }

    // Provide a method to get the price of a car
    pub fn get_price(&self, car: &str) -> Option<&u32> {
        self.car_prices.get(car)
    }

    pub fn get_all_prices(&self) -> &HashMap<String, u32> {
        &self.car_prices
    }
}