use ureq;

pub fn xdai_req() -> Result<(), ureq::Error> {
  let req: String = ureq::get("https://api.coingecko.com/api/v3/ping")
  .set("Example-Header", "header value")
  .call()?
  .into_string()?;
  println!("{:?}", req);
Ok(())
}

pub fn call_xdai_req() {
  let b = xdai_req();
  println!("{:?}", b);
}