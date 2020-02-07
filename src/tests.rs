use super::{rocket, Config};
use rocket::http::Status;
use rocket::local::Client;

#[test]
fn test_main() {
    let test_config = Config {
        secret: "thesecret".to_string(),
    };
    let client = Client::new(rocket(test_config)).unwrap();

    let mut response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);

    let body = response.body_string().unwrap();
    assert!(body.contains("kinderdom@mail.ru"));
}
