mod enums;
mod pattern_matchings;
mod generics;
mod traits;
mod built_in_traits;
fn main() {
    enums::run();
    pattern_matchings::run();
    generics::run();
    traits::run();
    built_in_traits::run();
}
