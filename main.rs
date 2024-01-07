use std::io;

fn main() {

    // Create mutable variable to assign temperature in Farenheit
    let mut temp = String::new();

    loop{
        println!("Enter temperature in Farenheit.");

        temp.clear();

        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read temp.");

        // Parse temp from String to floating-point number
        let temp: f32 = match temp
            .trim()
            .parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a valid number");
                continue;
                },
            };
        
        // Calculate celisuis conversion
        let celsius = (temp - 32.0) * 5.0/9.0;

        println!("The temperature in Celsius is {}", celsius);
        break;
    }

    
}
