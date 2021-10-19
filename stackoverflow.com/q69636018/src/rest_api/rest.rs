use crate::paas_core;
pub fn rest_caller() {
    println!("rest_caller() in rest_api/rest.rs called!");
    paas_core::create::create_func();
}
