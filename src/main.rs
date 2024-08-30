mod gemini;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "mlxai")]
struct Opt {
    model: String,
    api_key: String,
    text: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    let model = &opt.model;
    let api_key = &opt.api_key;
    let text = &opt.text;

    if model.starts_with("gemini") {
        gemini::request(api_key, model, text).await?;
    }
    Ok(())
}


