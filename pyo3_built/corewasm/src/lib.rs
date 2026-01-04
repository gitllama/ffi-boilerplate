
use extism_pdk::*;
use serde::Deserialize;

#[host_fn]
extern "ExtismHost" {
  fn wasm_log(n: String);
}

#[plugin_fn]
pub fn init() -> FnResult<()>{
  unsafe { wasm_log("called extism".to_string())? };
  Ok(())
}

#[plugin_fn]
pub fn set(value: i32) -> FnResult<()> {
  var::set("global", value)?;
  Ok(())
}

#[plugin_fn]
pub fn get() -> FnResult<i32> {
  // let mut c = var::get("vec")?.unwrap_or(0);
  let c = var::get("global")?.unwrap_or(0);
  Ok(c)
}


#[derive(Deserialize)]
struct AddArgs {
  left: i32,
  right: i32,
}

#[plugin_fn]
pub fn add(Json(args): Json<AddArgs>) -> FnResult<i32> {
  let dst = args.left + args.right;
  Ok(dst)
}



