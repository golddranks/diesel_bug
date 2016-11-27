use super::schema::*;



// Btw. as an aside it requires using ExpressionMethods here
// (try removing it and the use of sessions::id.eq() in the lib.rs won't compile), is that expected?
use diesel::ExpressionMethods;

#[derive(Debug, Insertable)]
#[table_name="sessions"]
pub struct NewSession<'a> {
    pub sess_token: &'a [u8],
    pub proposed_token: Option<&'a [u8]>,
    pub last_ip: Vec<u8>,
}

// Having "Identifiable" here triggers the bug, without it, it compiles just fine.
#[derive(Insertable, Identifiable, Debug, AsChangeset)]
#[table_name="sessions"]
#[changeset_options(treat_none_as_null = "true")]
pub struct UpdateSession<'a> {
    pub id: i32,
    pub sess_token: &'a [u8],
    pub proposed_token: Option<&'a [u8]>,
    pub last_ip: Vec<u8>,
}


#[derive(Insertable, Identifiable, Queryable, Debug)]
#[table_name="sessions"]
pub struct Session {
    pub id: i32,
    pub sess_token: Vec<u8>,
    pub proposed_token: Option<Vec<u8>>,
    pub last_ip: Vec<u8>,
}

