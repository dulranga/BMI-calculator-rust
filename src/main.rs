use std::{io, str::FromStr};

fn main() {
    let mut data: Vec<BMI> = vec![];

    println!("---------------------");
    let total_students = get_input("Number of Students", 0);

    for i in 0..total_students {
        let weight = get_input("Enter Weight (kg)", 0.0);
        let height = get_input("Enter Height (cm)", 0.0);
        println!("weight: {}, height:{}", weight, height);
        data.push(BMI {
            height,
            weight,
            idx: i,
        });
    }

    for bmi in &data {
        println!("BMI of {} is {}", bmi.idx, bmi.generate())
    }
    println!("Average BMIs {}", BMI::average(&data));
}

#[derive(Debug)]
struct BMI {
    weight: f32,
    height: f32,
    idx: i32,
}
impl BMI {
    fn generate(&self) -> f32 {
        self.weight / ((self.height / (100.0)).powf(2.0))
    }
    fn average(list_of_bmi: &Vec<Self>) -> f32 {
        let total = list_of_bmi
            .iter()
            .map(|bmi| bmi.generate())
            .reduce(|a, c| a + c);

        let total = match total {
            Some(t) => t,
            None => 0.0,
        };
        total / list_of_bmi.len() as f32
    }
}

fn get_input<T: FromStr>(msg: &str, default: T) -> T {
    println!("{}", msg);

    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(..) => {}
        Err(e) => println!("{}", e),
    };

    let value = match input.trim().parse() {
        Ok(students) => students,
        Err(_e) => {
            return default;
        }
    };

    value
}
