mod gemini;
mod azure;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "mlxai")]
struct Opt {
    model: String,
    key: String,
    text: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    let model = &opt.model;
    let api_key = &opt.key;
    let text = &opt.text;

    if model.starts_with("gemini") {
        gemini::request(api_key, model, text).await?;
    } else {
        azure::request(model, api_key, text).await?;
    }
    Ok(())
}


