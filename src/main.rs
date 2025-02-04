mod model {
    pub mod car;
}

mod controllers {
    pub mod cars_controller;
}

use model::car::Car;
use controllers::cars_controller::CarsController;

fn main() {
    let mut controller = CarsController::new();
 
    // Adding cars
    controller.add_car(Car {
        make: String::from("Toyota"),
        model: String::from("Corolla"),
        year: 2007,
    });
    controller.add_car(Car {
        make: String::from("Honda"),
        model: String::from("Civic"),
        year: 2010,
    });
    controller.add_car(Car {
        make: String::from("Ford"),
        model: String::from("Fiesta"),
        year: 2005,
    });

    println!("Original cars: {:?}", controller.get_cars());

    // Sort cars
    controller.sort_cars();
    println!("Sorted cars: {:?}", controller.get_cars());

    // Reverse cars
    controller.reverse_cars();
    println!("Reversed cars: {:?}", controller.get_cars());

    // Print car details
    for car in controller.get_cars() {
        println!("Car make: {}, model: {}, year: {}", car.make, car.model, car.year);
    }
}