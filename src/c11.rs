
use axum::{
  extract::Multipart,
  http::StatusCode,
  response::IntoResponse,
  routing::{get, post},
  Router,
};
use image::{self, GenericImageView, Pixel};
use tower_http::services::ServeDir;
use tracing::{debug, info};

// part 2:
/// POST endpoint /11/red_pixels, that takes in a PNG image in the image field of a multipart POST
/// request, and returns the number of pixels in the image that are perceived as "magical red" when
/// viewed with Santa's night vision goggles. The goggles considers a pixel "magical red" if the
/// color values of the pixel fulfill the formula red > blue + green.
///
/// curl -X POST http://localhost:8000/11/red_pixels \
///   -H 'Content-Type: multipart/form-data' \
///   -F 'image=@decoration.png' # the image from Task 1
///
/// 73034
// https://github.com/tokio-rs/axum/blob/d703e6f97a0156177466b6741be0beac0c83d8c7/examples/multipart-form/src/main.rs
pub async fn red_pixels(mut multipart: Multipart) -> impl IntoResponse {
  let mut count = 0;
  while let Some(field) = multipart.next_field().await.unwrap() {
    let name = field.name().unwrap().to_string();
    let content_type = field.content_type().unwrap_or("").to_string();
    let image_bytes = &field.bytes().await.unwrap();
    info!("Length of `{name}` with type {content_type} is {} bytes", image_bytes.len());

    if content_type == "image/png" {
      // Decode the image from bytes
      let img = image::load_from_memory(image_bytes).expect("Failed to decode image");

      // Iterate over pixels
      for (_x, _y, pixel) in img.pixels() {
        let rgb = pixel.to_rgb();
        let (r, g, b) = (rgb[0] as u32, rgb[1] as u32, rgb[2] as u32);
        if r > g + b {
          // info!("r: {}, g: {}, b: {}", r, g, b);
          count += 1;
        }
        // if count > 10 {
        //   break;
        // }
      }
    }
  }

  count.to_string()
}

// part 1:
// curl -I -X GET http://localhost:8000/11/assets/decoration.png
//
// ```
// HTTP/1.1 200 OK
// content-type: image/png
// content-length: 787297
// ```
// use multipart to handle file upload
// axum static file example: https://github.com/shuttle-hq/shuttle-examples/blob/main/axum/static-files/src/main.rs
