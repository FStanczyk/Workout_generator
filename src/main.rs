use exercise::Training;

mod exercise;
mod filters;
mod generator;
mod user;

fn main() {
    let c = filters::filter_by_attribute_simple(10, 8, None, Some(1)).expect("not work");

    let wow = generator::generate_week_plan(generator::GOAL::Bulk, 4, 8, &c);

    for i in wow.iter() {
        Training::display(i);
        println!("------------NEXT DAY------------------")
    }

}
