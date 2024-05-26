mod map;

fn main() {
    // Create a new Cars struct
    let cars = map::Cars::new();

    // Get the price of a Toyota Camry
    if let Some(price) = cars.get_price("Toyota Camry") {
        println!("The price of a Toyota Camry is ${}", price);
    }

    let all_prices = cars.get_all_prices();

    // Iterate over the car prices and print them
    for (car, price) in all_prices {
        println!("The price of a {} is ${}", car, price);
    }
}