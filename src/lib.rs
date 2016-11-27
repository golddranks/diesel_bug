#![feature(proc_macro, custom_derive, custom_attribute, plugin)]
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;

pub mod schema;
pub mod models;

#[cfg(test)]
mod tests {
    use schema::sessions;
    #[test]
    fn it_works() {
        let sess: Session = sessions::table
            .filter(sessions::id.eq(1))
            .get_result().unwrap();
    }
}
