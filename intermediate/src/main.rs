mod enums;
mod pattern_matchings;
mod generics;
mod traits;
mod built_in_traits;
mod error_handling;

fn main() {
    enums::run();
    pattern_matchings::run();
    generics::run();
    traits::run();
    built_in_traits::run();
    // error_handling::run();
}
