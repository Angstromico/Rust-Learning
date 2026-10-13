mod basic_variables;
mod compiler_directive_allow;
mod constants;
mod mutability;
mod scope_variables;
mod shadowing_variables;
mod type_aliases;

fn main() {
    basic_variables::run();
    mutability::run();
    shadowing_variables::run();
    scope_variables::run();
    constants::run();
    type_aliases::run();
    compiler_directive_allow::run();
}
