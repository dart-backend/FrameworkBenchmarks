use crate::*;

pub async fn request(controller_data: ControllerData) {
    let _ = controller_data
        .set_response_header(CONNECTION, CONNECTION_KEEP_ALIVE)
        .await
        .set_request_header(CONTENT_TYPE, APPLICATION_JSON)
        .await
        .set_response_status_code(200)
        .await;
}
