mod rest;

pub fn rest_func() {
    println!("rest_func() in rest_api/mod.rs called!");
    rest::rest_caller();
}
