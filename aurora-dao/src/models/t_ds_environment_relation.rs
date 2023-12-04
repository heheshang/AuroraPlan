use chrono::NaiveDateTime;
use diesel::prelude::*;

use super::DPool;

#[derive(Clone, Debug, PartialEq, Queryable, Selectable, Identifiable, Default)]
#[diesel(table_name = crate::schema::t_ds_environment_relation)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct EnvironmentRelation {
    pub id: i32,
    pub code: i64,
    pub name: Option<String>,
    pub config: Option<String>,
    pub description: Option<String>,
    pub operator: Option<i32>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
    pub worker_groups: Option<Vec<Option<String>>>,
}

impl EnvironmentRelation {
    pub async fn find_by_code(_id: i32, pool: DPool) -> QueryResult<Self> {
        use crate::schema::t_ds_environment_relation::dsl::*;
        // t_ds_environment_relation
        //     .filter(id.eq(_id))

        //     .select(t_ds_environment_relation::all_columns())
        //     .first::<Self>(conn)
        let results = t_ds_environment_relation
            .filter(id.eq(_id))
            .select(Self::as_select())
            .first::<Self>(&mut pool.get().unwrap())
            .unwrap_or_default();
        Ok(results)
    }
}
