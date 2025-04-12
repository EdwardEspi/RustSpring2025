use std::{fs::File, io::Write, thread, time::Duration};
use serde::Deserialize;

// Define the Pricing trait
trait Pricing {
    fn fetch_price(&self) -> Result<f64, String>;
    fn save_to_file(&self, price: f64);
}

// Define the Bitcoin struct
struct Bitcoin;

// Define the Ethereum struct
struct Ethereum;

// Define the SP500 struct
struct SP500;

// Implement the Pricing trait for Bitcoin
impl Pricing for Bitcoin {
    fn fetch_price(&self) -> Result<f64, String> {
        let api_key = "745a6f5dd812823faff89749e0845f1fdadcd17dc997e04aef77f80de1ab258d";
        let url = format!("https://min-api.cryptocompare.com/data/price?fsym=BTC&tsyms=USD&api_key={}", api_key);
        let response = ureq::get(&url)
            .call()
            .map_err(|e| e.to_string())?;
        let response_text = response.into_string().map_err(|e| e.to_string())?;
        let data: CryptoCompareResponse = serde_json::from_str(&response_text).map_err(|e| e.to_string())?;
        eprintln!("Bitcoin Price: ${:.2}", data.usd); // Log only the price
        Ok(data.usd)
    }

    fn save_to_file(&self, price: f64) {
        let mut file = File::create("bitcoin.txt").expect("Failed to create file");
        writeln!(file, "Bitcoin Price: ${:.2}", price).expect("Failed to write to file");
    }
}

// Implement the Pricing trait for Ethereum
impl Pricing for Ethereum {
    fn fetch_price(&self) -> Result<f64, String> {
        let api_key = "745a6f5dd812823faff89749e0845f1fdadcd17dc997e04aef77f80de1ab258d";
        let url = format!("https://min-api.cryptocompare.com/data/price?fsym=ETH&tsyms=USD&api_key={}", api_key);
        let response = ureq::get(&url)
            .call()
            .map_err(|e| e.to_string())?;
        let response_text = response.into_string().map_err(|e| e.to_string())?;
        let data: CryptoCompareResponse = serde_json::from_str(&response_text).map_err(|e| e.to_string())?;
        eprintln!("Ethereum Price: ${:.2}", data.usd); // Log only the price
        Ok(data.usd)
    }

    fn save_to_file(&self, price: f64) {
        let mut file = File::create("ethereum.txt").expect("Failed to create file");
        writeln!(file, "Ethereum Price: ${:.2}", price).expect("Failed to write to file");
    }
}

// Implement the Pricing trait for SP500
impl Pricing for SP500 {
    fn fetch_price(&self) -> Result<f64, String> {
        let url = "https://query1.finance.yahoo.com/v8/finance/chart/%5EGSPC?interval=1m";
        let response = ureq::get(url)
            .call()
            .map_err(|e| e.to_string())?;
        let response_text = response.into_string().map_err(|e| e.to_string())?;
        let data: YahooFinanceResponse = serde_json::from_str(&response_text).map_err(|e| e.to_string())?;
        
        // Extract the latest close price
        if let Some(close_prices) = data.chart.result.first().and_then(|r| r.indicators.quote.first()).map(|q| &q.close) {
            if let Some(latest_price) = close_prices.iter().copied().flatten().last() {
                eprintln!("S&P 500 Index Price: ${:.2}", latest_price); // Log only the price
                return Ok(latest_price);
            }
        }
        Err("Failed to fetch S&P 500 index price".to_string())
    }

    fn save_to_file(&self, price: f64) {
        let mut file = File::create("sp500.txt").expect("Failed to create file");
        writeln!(file, "S&P 500 Index Price: ${:.2}", price).expect("Failed to write to file");
    }
}

// Structs for deserializing JSON responses
#[derive(Deserialize)]
struct CryptoCompareResponse {
    #[serde(rename = "USD")]
    usd: f64,
}

#[derive(Deserialize)]
struct YahooFinanceResponse {
    chart: Chart,
}

#[derive(Deserialize)]
struct Chart {
    result: Vec<ResultData>,
}

#[derive(Deserialize)]
struct ResultData {
    indicators: Indicators,
}

#[derive(Deserialize)]
struct Indicators {
    quote: Vec<Quote>,
}

#[derive(Deserialize)]
struct Quote {
    close: Vec<Option<f64>>,
}

fn main() {
    let assets: Vec<Box<dyn Pricing>> = vec![
        Box::new(Bitcoin),
        Box::new(Ethereum),
        Box::new(SP500),
    ];

    loop {
        for asset in &assets {
            match asset.fetch_price() {
                Ok(price) => asset.save_to_file(price),
                Err(e) => eprintln!("Error fetching price: {}", e),
            }
        }
        thread::sleep(Duration::from_secs(10));
    }
}