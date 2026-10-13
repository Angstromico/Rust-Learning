mod basic_variables;
mod constants;
mod mutability;
mod scope_variables;
mod shadowing_variables;

fn main() {
    basic_variables::run();
    mutability::run();
    shadowing_variables::run();
    scope_variables::run();
    constants::run();
}
