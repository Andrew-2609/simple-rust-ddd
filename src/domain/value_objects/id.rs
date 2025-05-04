use diesel::{deserialize::FromSql, pg::Pg, serialize::ToSql, sql_types::Integer};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub enum ID {
    New,
    Existing(i32),
}

impl From<i32> for ID {
    fn from(value: i32) -> Self {
        Self::Existing(value)
    }
}

impl FromSql<Integer, Pg> for ID {
    fn from_sql(
        bytes: <Pg as diesel::backend::Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        let id = <i32 as FromSql<Integer, Pg>>::from_sql(bytes)?;
        Ok(ID::Existing(id))
    }
}

impl ToSql<Integer, Pg> for ID {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, Pg>,
    ) -> diesel::serialize::Result {
        match self {
            ID::New => Err("Cannot serialize UserId".into()),
            ID::Existing(id) => <i32 as ToSql<Integer, Pg>>::to_sql(id, out),
        }
    }
}

impl From<ID> for Option<i32> {
    fn from(value: ID) -> Self {
        match value {
            ID::New => None,
            ID::Existing(id) => Some(id),
        }
    }
}
