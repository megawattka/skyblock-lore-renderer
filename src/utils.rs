use std::io::Cursor;

use base64::{Engine as _, engine::general_purpose::STANDARD};
use bytes::Bytes;
use image::ImageFormat;
use warp::reject::{self, Rejection};
use warp::http::{Response, StatusCode};

use crate::errors::BadRequest;

pub fn to_png_bytes(img: &image::RgbImage) -> Vec<u8> {
    let mut cursor = Cursor::new(Vec::new());
    img.write_to(&mut cursor, ImageFormat::Png)
        .expect("encode failed");
    cursor.into_inner()
}

pub fn decode_base64(body: &Bytes) -> Result<Vec<u8>, Rejection> {
    STANDARD.decode(body)
        .map_err(|_| reject::custom(BadRequest("base64 decode failed")))
}

pub fn bytes_to_string(bytes: Vec<u8>) -> Result<String, Rejection> {
    String::from_utf8(bytes)
        .map_err(|_| reject::custom(BadRequest("decoded data contains invalid utf8")))
}

pub fn build_image_response(content: Vec<u8>, elapsed: String) -> Response<Vec<u8>> {
    Response::builder()
        .status(StatusCode::OK)
        .header("x-elapsed-ms", elapsed)
        .header("content-type", "image/png")
        .body(content)
        .expect("Failed to create response")
}