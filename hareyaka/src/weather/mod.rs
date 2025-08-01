use crate::weather::utils::ser_structs::CurrentData;

mod new;
mod to_json;
mod utils;

pub struct Hareyaka {
    data: CurrentData,
}