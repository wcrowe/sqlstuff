docker-compose.yaml
version: '3.9'

services:
  mssql:
    image: mcr.microsoft.com/mssql/server:2019-latest
    ports:
      - 1433:1433
    volumes:
      - ~/apps/mssql/data:/var/lib/mssqlql/data
    environment:
      - ACCEPT_EULA=Y
      - SA_PASSWORD=mssql1Ipw


createtable.sql

CREATE DATABASE Work;
go

IF  EXISTS (SELECT * FROM sys.objects WHERE object_id = OBJECT_ID(N'[dbo].[TestRow]') AND type in (N'U'))
DROP TABLE [dbo].[TestRow]
GO
 CREATE TABLE [TestRow](
            [Id] [int] NOT NULL PRIMARY KEY,
            [VarCharRow] [varchar] (50) NULL,
            [NVarCharRow] [nvarchar](15) NULL,
            [UuidRow] [uniqueidentifier] NULL,
            [LongRow] [bigint] NULL,
            [DateTimeRow] [datetime] NULL,
            [SmallIntRow] [smallint] NULL,
            [BitRow] [bit] NULL,
            [FloatRow] [float](24) NULL,
            [DoubleRow] [float](53) NULL,
            [RealRow] [real] NULL,
            [ImageRow] [image] NULL
        );

 INSERT INTO [TestRow] 
    (Id, VarCharRow, NVarCharRow, UuidRow, LongRow, DateTimeRow, SmallIntRow, BitRow, FloatRow, DoubleRow, RealRow) 
    VALUES 
    (1, 'varchar', 'nvarchar', '89e022ce-d3b6-43a7-a359-4618571487a6', 9999999999999999, '2021-01-01', 2, 1, 10.123123125, 99.1231231258, 10.5)

    INSERT INTO [TestRow] 
    (Id, VarCharRow, NVarCharRow, UuidRow, LongRow, DateTimeRow, SmallIntRow, BitRow, FloatRow, DoubleRow, RealRow) 
    VALUES 
    (2, 'first', 'last',NEWID(), 99999999999999229, Getdate(), 4, 4, 15.123123125, 299.1231231258, 420.00)

/**********************  Linqpad *********************************/












/**********************  Rust  utility *********************************/
mod util;
use crate::util::connect_localhost;

#[ignore = "manual"]
#[tokio::test]
async fn test_database_up() {
    let mut client = connect_localhost().await.unwrap();

    client
        .execute(r#"CREATE DATABASE TiberiusDeriveTest;"#, &[])
        .await
        .ok();

    let query = r#"
        CREATE TABLE [TiberiusDeriveTest].[dbo].[TestRow](
            [Id] [int] NOT NULL PRIMARY KEY,
            [VarCharRow] [varchar] (50) NULL,
            [NVarCharRow] [nvarchar](15) NULL,
            [UuidRow] [uniqueidentifier] NULL,
            [LongRow] [bigint] NULL,
            [DateTimeRow] [datetime] NULL,
            [SmallIntRow] [smallint] NULL,
            [BitRow] [bit] NULL,
            [FloatRow] [float](24) NULL,
            [DoubleRow] [float](53) NULL,
            [RealRow] [real] NULL,
            [ImageRow] [image] NULL,
        );
    
        "#;

    client.execute(query, &[]).await.unwrap();
}

#[ignore = "manual"]
#[tokio::test]
async fn test_insert_rows() {
    let mut client = connect_localhost().await.unwrap();

    let test_row = r#"
    INSERT INTO [TiberiusDeriveTest].[dbo].[TestRow] 
    (Id, VarCharRow, NVarCharRow, UuidRow, LongRow, DateTimeRow, SmallIntRow, BitRow, FloatRow, DoubleRow, RealRow) 
    VALUES 
    (1, 'varchar', 'nvarchar', '89e022ce-d3b6-43a7-a359-4618571487a6', 9999999999999999, '2021-01-01', 2, 1, 10.123123125, 99.1231231258, 10.5)
    "#;

    let test_row_null = r#"
    INSERT INTO [TiberiusDeriveTest].[dbo].[TestRow] 
    (Id) 
    VALUES 
    (2)
    "#;

    client.execute(test_row, &[]).await.unwrap();
    client.execute(test_row_null, &[]).await.unwrap();
}

#[ignore = "manual"]
#[tokio::test]
async fn test_datebase_down() {
    let mut client = connect_localhost().await.unwrap();

    let query = r#"
    DELETE FROM [TiberiusDeriveTest].[dbo].[TestRow];
    DROP DATABASE TiberiusDeriveTest;
    "#;

    client.execute(query, &[]).await.unwrap();
}