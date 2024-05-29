mod semdata;
mod functions;

fn main() {
    println!("Welcome to SPI/CPI Calculator");    
    println!("=======================================\n");
    println!("Instructions: \n");
    println!("Enter n for nth semester");
    println!("Enter n.x for nth semester's xth option");
    println!("=======================================\n");
    println!("What do you want to Calculate? (SPI/CPI): ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    let input = input.trim();
    let input = input.to_uppercase();
    
    if input == "CPI"{
        println!("Calculating CPI");
        functions::calculate_cpi();
    }
    else if input == "SPI"{
        println!("Calculating SPI...\n");
        functions::calculate_spi();
    }
    else{
        println!("Invalid Input");
        println!("Exiting the Calculator");
        return;
    }
}
