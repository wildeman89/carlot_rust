#[derive(Debug)]
pub struct Sedan {
    brand: String,
    model: String,
    year: String,
    miles: u32,
}

impl Sedan {
    pub fn display(&self) {
        println!(
            "Brand: {}\nModel: {}\nYear: {}\nMileage: {}",
            self.brand, self.model, self.year, self.miles
        );
    }
}

pub fn create_sedan(brand: String, model: String, year: String, mileage: u32) -> Sedan {
    Sedan {
        brand,
        model,
        year,
        miles: mileage,
    }
}
