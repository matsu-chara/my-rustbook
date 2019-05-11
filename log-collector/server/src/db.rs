use diesel::insert_into;
use diesel::prelude::*;
use diesel::result::QueryResult;

use crate::model::*;

pub fn insert_log(cn: &PgConnection, log: &NewLog) -> QueryResult<i64> {
    use crate::schema::logs::dsl;

    insert_into(dsl::logs)
        .values(log)
        .returning(dsl::id)
        .get_result(cn)
}

pub fn insert_logs(cn: &PgConnection, log: &[NewLog]) -> QueryResult<Vec<i64>> {
    use crate::schema::logs::dsl;

    insert_into(dsl::logs)
        .values(log)
        .returning(dsl::id)
        .load(cn)
}
