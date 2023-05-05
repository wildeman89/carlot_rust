use crate::vehicles::sedan::create_sedan;
pub mod vehicles;

fn main() {
    println!("\n\tWelcome to CarLot!");

    let car1 = create_sedan(
        String::from("Ford"),
        String::from("Mustang"),
        String::from("1995"),
        56_987,
    );

    car1.display();
}
