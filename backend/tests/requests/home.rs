use backend::app::App;
use loco_rs::testing::prelude::*;
use serial_test::serial;


#[tokio::test]
#[serial]
async fn can_get_healthcheck() {
    request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/healthcheck").await;

        assert_eq!(res.status_code(), 200);
        let body = res.as_bytes();
        let parsed_response: serde_json::Value = serde_json::from_slice(body).unwrap();

        assert_eq!(
            parsed_response.get("message")
                .expect("Missing 'message' field")
                .as_str()
                .expect("'message' is not a string"),
            "Status is healthy!"
        );
        assert_eq!(
            parsed_response.get("status")
                .expect("Missing 'status' field")
                .as_u64()
                .expect("'status' is not an integer"),
            200
        );
    })
    .await;
}
