fn main() {
    println!("\n\tWelcome to CarLot!");

    let car1 = Vehicle {
        brand: String::from("Mazda"),
        model: String::from("CX-30"),
        year: String::from("2022"),
        miles: 16_102,
    };

    car1.display();
}

#[derive(Debug)]
struct Vehicle {
    brand: String,
    model: String,
    year: String,
    miles: u32,
}

impl Vehicle {
    fn display(&self) {
        println!(
            "Brand: {}\nModel: {}\nYear: {}\nMileage: {}",
            self.brand, self.model, self.year, self.miles
        );
    }
}
