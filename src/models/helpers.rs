// use std::{error::Error, fmt::Display};

use std::error::Error;

use sqlx::{
  database::HasValueRef,
  error::BoxDynError,
  postgres::{PgTypeInfo, PgValueRef},
  Decode, PgPool, Postgres,
};

use ulid::Ulid;

// // SQLx
#[derive(Clone, PartialEq, Eq, Debug)]
// #[sqlx(transparent)]
pub struct SqlxUlid(Ulid);

// example DAO
pub struct Comments {
  pub id: SqlxUlid,
}

impl<'r> sqlx::Decode<'r, Postgres> for SqlxUlid
where &'r str: sqlx::Decode<'r, Postgres>
{
  fn decode(
    value: <Postgres as HasValueRef<'r>>::ValueRef,
  ) -> Result<SqlxUlid, Box<dyn Error + 'static + Send + Sync>> {
    let value: &str = <&str as sqlx::Decode<Postgres>>::decode(value)?;

    Ok(SqlxUlid(Ulid::from_string(value)?))
  }
}

async fn query(pool: &PgPool) {
  let ulid = SqlxUlid(Ulid::new());

  let record = sqlx::query!(
    r#"SELECT id as "ulid: SqlxUlid" FROM comments WHERE id = $1"#,
    "id".to_string()
  )
  .fetch_all(pool)
  .await
  .unwrap();

   for r in record {
    let namespace = Comments { id: r.ulid };
   }

//   let namespace = Comments { id: record.id };
}
