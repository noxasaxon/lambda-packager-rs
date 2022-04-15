// #![deny(clippy::all)]
// use futures::prelude::*;
// use napi::bindgen_prelude::*;
use napi_derive::napi;
// use tokio::fs;

#[cfg(all(
  any(windows, unix),
  target_arch = "x86_64",
  not(target_env = "musl"),
  not(debug_assertions)
))]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

// #[napi]
// pub fn plus_100(input: u32) -> u32 {
//   input + 100
// }

// #[napi]
// async fn read_file_async(path: String) -> Result<Buffer> {
//   fs::read(path)
//     .map(|r| match r {
//       Ok(content) => Ok(content.into()),
//       Err(e) => Err(Error::new(
//         Status::GenericFailure,
//         format!("failed to read file, {}", e),
//       )),
//     })
//     .await
// }

use futures::prelude::*;
use napi::bindgen_prelude::*;
use tokio::fs;

#[napi]
async fn read_file_async(path: String) -> Result<Buffer> {
  fs::read(path)
    .map(|r| match r {
      Ok(content) => Ok(content.into()),
      Err(e) => Err(Error::new(
        Status::GenericFailure,
        format!("failed to read file, {}", e),
      )),
    })
    .await
}
