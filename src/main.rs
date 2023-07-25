use ethers::{
    contract::abigen,
    core::types::Address,
    providers::{Provider, StreamExt, Ws},
};
use eyre::Result;
use std::sync::Arc;

abigen!(IERC20, "./IUniswapV2Pair.json");

const WSS_URL: &str = "wss://mainnet.infura.io/ws/v3/c60b0bb42f8a4c6481ecd229eddaca27";
const WETH_ADDRESS: &str = "0x0d4a11d5eeaac28ec3f61d100daf4d40471f1852"; //v2 eth/usdt

#[tokio::main]
async fn main() -> Result<()> {
    let provider = Provider::<Ws>::connect(WSS_URL).await?;
    let client = Arc::new(provider);
    let address: Address = WETH_ADDRESS.parse()?;
    let contract = IERC20::new(address, client);

    let events = contract.events().from_block(16232696);
    let mut stream = events.stream().await?;

    loop {
        let event = stream.next().await.unwrap();

        match event {
            Ok(IERC20Events::SyncFilter(f)) => {
                let price = (f.reserve_1 as f64 / f.reserve_0 as f64) * 1_000_000_000_000.0; //eth e-18 & usdt e-6
                println!("Uniswap ETH price: '{}' USDt", price);
            }
            Ok(_) => {},
            Err(e) => println!("error: {}", e),
        }
    }
}