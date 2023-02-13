extern crate rand;
use rand::seq::SliceRandom;

fn main() {
    let diets = vec![
        "Vegetarian",
        "Mediterranean",
        "Ketogenic",
        "Paleo",
        "Whole30",
        "Intermittent Fasting",
        "Low-Carb",
        "Low-Fat",
    ];
    let number_of_diets = 3;

    println!("Randomly Selected Diets:");
    let selected_diets: Vec<&str> = diets
        .choose_multiple(&mut rand::thread_rng(), number_of_diets)
        .collect();
    for (index, diet) in selected_diets.iter().enumerate() {
        println!("{}. {}", index + 1, diet);
    }
}
