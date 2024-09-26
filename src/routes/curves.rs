// use actix_web::{web, Result};
// use async_std::net::TcpStream;
// use serde::Deserialize;
// use std::error::Error;

// #[derive(Deserialize)]
// pub struct FormData {
//     email: String,
//     name: String,
// }

// pub async fn GetCurves(
//     form: web::Form<FormData>,
//     pool: web::Data<bb8::Pool<bb8_tiberius::ConnectionManager>>,
// ) -> Result<String, Box<dyn Error>> {
//     print!("{} {}", form.email, form.name);
//     let mut conn = pool.get().await?;
//     let res = conn
//         .simple_query("SELECT @@version")
//         .await?
//         .into_first_result()
//         .await?
//         .into_iter()
//         .map(|row| {
//             let val: &str = row.get(0).unwrap();
//             String::from(val)
//         })
//         .collect::<Vec<_>>();

//     println!("{:?}", &res);
//     Ok(format!("Welcome {}!", form.name))
// }
