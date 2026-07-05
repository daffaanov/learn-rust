#[allow(unused_variables, unused_assignments, dead_code)]
const TOUCHDOWN_POINTS: i32 = 6;

#[allow(unused_variables, unused_assignments)]
fn main() {
    let season = "Summer";
    let mut points_scored = 28;

    let event_time = "06:00";
    let event_time = 6;
    points_scored = 35;

    println!(
        "Points: {0}, Season: {1}, Event Time: {2}, Touchdown Points: {3}",
        points_scored, season, event_time, TOUCHDOWN_POINTS
    );

    let favorite_beverage = "Coffee";
    println!("Favorite Beverage: {favorite_beverage}");
}
