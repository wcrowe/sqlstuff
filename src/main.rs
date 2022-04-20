#![allow(unused_imports)]
#![allow(dead_code)]
use once_cell::sync::Lazy;
use std::env;
use chrono::NaiveDateTime;
use tiberius::Uuid;
use tiberius_derive::FromRow;

#[derive(FromRow, Debug, Clone, PartialEq)]
#[tiberius_derive(owned)]
struct TestRow {
    pub id: i32,
    pub var_char_row: String,
    pub n_var_char_row: String,
    pub uuid_row: Uuid,
    pub long_row: i64,
    pub date_time_row: chrono::NaiveDateTime,
    pub small_int_row: i16,
    pub bit_row: bool,
    pub float_row: f32,
    pub double_row: f64,
    pub real_row: f32,
}

#[allow(non_snake_case)]
#[derive(FromRow, Debug, Clone, PartialEq, Default)]
#[tiberius_derive(owned)]
struct TestRowNullable {
    pub Id: i32,
    pub VarCharRow: Option<String>,
    pub NVarCharRow: Option<String>,
    pub UuidRow: Option<Uuid>,
    pub LongRow: Option<i64>,
    pub DateTimeRow: Option<chrono::NaiveDateTime>,
    pub SmallIntRow: Option<i16>,
    pub BitRow: Option<bool>,
    pub FloatRow: Option<f32>,
    pub DoubleRow: Option<f64>, // borked. Breaks because tiberius inteprets a a nullable float field as F32(None)
    pub RealRow: Option<f32>,
}

static CONN_STR: Lazy<String> = Lazy::new(|| {
    env::var("TIBERIUS_TEST_CONNECTION_STRING").unwrap_or_else(|_| {
        "server=tcp:host.docker.internal,1433;User ID=sa;Password=mssql1Ipw;Pooling=True;TrustServerCertificate=true;".to_owned()
    })
});

#[cfg(not(all(windows, feature = "sql-browser-tokio")))]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  
    use bb8::Pool;
    use bb8_tiberius::ConnectionManager;

    let query = r"
    SELECT
        [Id],[VarCharRow],[NVarCharRow],[UuidRow],[LongRow],[DateTimeRow],[SmallIntRow],[BitRow],[FloatRow],[DoubleRow],[RealRow]
    FROM 
        [Work].[dbo].[TestRow]
    WHERE VarCharRow is not null
    ORDER BY ID
        ";


    let mgr = ConnectionManager::build(CONN_STR.as_str())?;

    let pool = Pool::builder().max_size(2).build(mgr).await?;

    let mut conn = pool.get().await?;

    let rows = conn
        .simple_query(query)
        .await?
        .into_first_result()
        .await?;

    let rows = rows
        .into_iter()
        .map(TestRowNullable::from_row)
        .collect::<Result<Vec<_>, _>>()?;

    println!("{:?}", rows);

    Ok(())
}

// #[cfg(all(windows, feature = "sql-browser-tokio"))]
// #[tokio::main]
// async fn main() -> anyhow::Result<()> {
//     use tiberius::SqlBrowser;

//     let config = Config::from_ado_string(&CONN_STR)?;

//     let tcp = TcpStream::connect_named(&config).await?;
//     tcp.set_nodelay(true)?;

//     let mut client = Client::connect(config, tcp.compat_write()).await?;

//     let stream = client.query("SELECT @P1", &[&1i32]).await?;
//     let row = stream.into_row().await?.unwrap();

//     println!("{:?}", row);
//     assert_eq!(Some(1), row.get(0));

//     Ok(())
// }
