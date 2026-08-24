pub mod dashboard;
pub mod models;
pub mod repositories;
pub mod store;

pub fn model_set() -> toasty::ModelSet {
    toasty::models!(crate::*)
}
