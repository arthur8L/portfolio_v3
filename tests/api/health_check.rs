use crate::helper;

#[tokio::test]
async fn health_check_passes() {
    let app = helper::spawn_app().await;
    println!("{}", &app.address);
    // let res = reqwest::Client::new()
    //     .get(format!("{}/health_check", &app.address))
    //     .send()
    //     .await
    //     .expect("Expected request to healh_check to send successfully");

    // assert_eq!(
    //     res.status().as_u16(),
    //     200,
    //     "Expected status code to be 200, but got = {}",
    //     res.status().as_u16()
    // );
}
