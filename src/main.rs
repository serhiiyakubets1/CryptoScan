mod blockchains;
use reqwest::get;
use blockchains::{
    binance,
    cardano,
    ethereum,
    fantom,
    harmony,
    huobi,
    polygon,
    solana,
    xdai
};


pub fn main() {
    let req1 = reqwest::get("https://api.coingecko.com/api/v3/ping");

    // binance::binance_req();
    // cardano::cardano_req();
    // ethereum::ethereum_req();
    // fantom::fantom_req();
    // harmony::harmony_req();
    // huobi::huobi_req();
    // polygon::polygon_req();
    // solana::solana_req();
    // xdai::xdai_req();
}
