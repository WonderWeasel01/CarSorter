use crate::model::car::Car;

pub struct CarsController {
    cars: Vec<Car>,
}

impl CarsController {
    pub fn new() -> Self {
        Self { cars: Vec::new() }
    }

    pub fn add_car(&mut self, car: Car) {
        self.cars.push(car);
    }

    pub fn sort_cars(&mut self) {
        self.cars.sort_by(|a, b| a.year.cmp(&b.year)); // Sort by year
    }

    pub fn reverse_cars(&mut self) {
        self.cars.reverse();
    }

    pub fn get_cars(&self) -> &Vec<Car> {
        &self.cars
    }
}