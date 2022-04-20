use once_cell::sync::Lazy;
use std::env;

use tiberius::Uuid;
use tiberius_derive::FromRow;

#[derive(FromRow, Debug, Clone, PartialEq)]
#[tiberius_derive(owned)]
struct TestRow {
    pub id: Uuid,
    pub FirstName: String,
    pub LastName: String,
    pub dec_col: bigdecimal,
    pub num_col: bigdecimal,
    pub dbl_col: f64,
    pub createdate: chrono::NaiveDateTime,
}

#[derive(FromRow, Debug, Clone, PartialEq)]
#[tiberius_derive(owned)]
struct TestRowNullable {
    pub id: Uuid,
    pub FirstName: Option<String>,
    pub LastName: Option<String>,
    pub dec_col: Option<bigdecimal>,
    pub num_col: Option<bigdecimal>,
    pub dbl_col: Option<f64>,
    pub createdate: Option<chrono::NaiveDateTime>,
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
        .map(TestRow::from_row)
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
