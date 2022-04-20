use once_cell::sync::Lazy;
use std::env;
use tiberius::time::chrono;
use tiberius::Uuid;
// use tokio::net::TcpStream;
// use tokio_util::compat::TokioAsyncWriteCompatExt;
use tiberius_derive::FromRow;
#[derive(FromRow, Debug)]
#[tiberius_derive(owned)]
struct TestRowNullable {
    pub id: Uuid,
    pub FirstName: Option<String>,
    pub LastName: Option<String>,
    pub dec_col: Option<f32>,
    pub num_col: Option<f32>,
    pub dbl_col: Option<f64>,
    pub createdate: chrono::NaiveDateTime,
    // pub small_int_row: i16,
    // pub bit_row: bool,
    // pub float_row: f32,
    // pub double_row: f64,
    // pub real_row: f32,
}

static CONN_STR: Lazy<String> = Lazy::new(|| {
    env::var("TIBERIUS_TEST_CONNECTION_STRING").unwrap_or_else(|_| {
        "server=tcp:localhost\\SQLEXPRESS,4119;IntegratedSecurity=true;TrustServerCertificate=true".to_owned()
    })
});

#[cfg(not(all(windows, feature = "sql-browser-tokio")))]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let conn_str = std::env::var(CONN_STR)?;

    use bb8::Pool;
    use bb8_tiberius::ConnectionManager;
    use tiberius::Uuid;

    let mgr = ConnectionManager::build(CONN_STR.as_str())?;

    let pool = Pool::builder().max_size(2).build(mgr).await?;

    let mut conn = pool.get().await?;

    let rows = conn
        .simple_query(
            r#"SELECT [id]
      ,[FirstName]
      ,[LastName]
      ,[dec_col]
      ,[num_col]
      ,[dbl_col]
      ,[createdate]
        FROM [work].[dbo].[TestDataTypes]"#,
        )
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
