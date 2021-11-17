mod guess_game;
mod data_type;
mod function;
mod control_flow;
mod ownership;
mod rust_struct;
mod rust_enum;

fn main() {
    //guess_game::guess();
    //data_type::learn_data_type();
    //function::rust_function();
    //control_flow::condition();
    //ownership::ownership();
    //rust_struct::rust_struct();
    rust_enum::rust_enum();

    // const is determined at compilation, so it can be access at any time within the scope
    //println!("{}", MY_CONST);
    //const MY_CONST: i32 = 1;
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}