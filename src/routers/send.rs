use crate::model::send_body::MessagePayload;
use actix_web::{post, web, HttpResponse, Responder};
use serenity::http::Http;
use serenity::model::channel::Channel;
use serenity::model::id::ChannelId;
use std::sync::Arc;
#[post("/send")]
pub async fn send_message(
    payload: web::Json<MessagePayload>,
    http: web::Data<Arc<Http>>,
    auth: web::Data<String>,
) -> impl Responder {
    let pay_auth = &payload.auth;
    if pay_auth != auth.get_ref() {
        return HttpResponse::BadRequest().body("auth failed");
    }
    let channel_id = ChannelId::new(payload.channel_id);
    let channel_res = channel_id.to_channel(&http.as_ref()).await;
    if channel_res.is_err() {
        return HttpResponse::InternalServerError().body("Failed to retrieve channel information");
    }
    let channel = channel_res.unwrap();
    if let Channel::Guild(guild_channel) = channel {
        if !guild_channel.is_text_based() {
            return HttpResponse::BadRequest()
                .body("The specified channel is not a text-based channel");
        }
    } else {
        return HttpResponse::BadRequest().body("The specified channel is not a guild channel");
    }

    let content = &payload.content;
    if channel_id.say(http.as_ref(), content).await.is_err() {
        return HttpResponse::InternalServerError().body("Failed to send message");
    }

    HttpResponse::Ok().body("Message sent")
}
