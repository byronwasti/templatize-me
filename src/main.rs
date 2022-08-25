use anyhow::Result;
use clap::Parser;
use serde_json::Value;
use std::fs::read_to_string;
use std::path::PathBuf;
use tera::{Context, Tera};

#[derive(Parser, Debug)]
#[clap()]
struct Args {
    #[clap(short, long, default_value = "template")]
    template: PathBuf,

    #[clap(short, long, default_value = "context.toml")]
    context: Vec<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let context = load_context(&args.context)?;
    let template = read_to_string(args.template)?;

    let res = Tera::one_off(&template, &Context::from_serialize(&context)?, true)?;
    print!("{res}");

    Ok(())
}

fn load_context(paths: &[PathBuf]) -> Result<Value> {
    let mut all_context = Value::Null;
    for path in paths {
        let context = std::fs::read_to_string(&path).unwrap_or_else(|_| "".to_string());
        let extension = path.extension().map(|s| s.to_str().unwrap().to_string());

        let context: Value = match extension.as_deref() {
            Some("toml") => {
                // NOTE: There is _probably_ a better way of
                // doing this, but converting from toml::Value
                // to json::Value works like this.
                let context = context.parse::<toml::Value>()?;
                let context = serde_json::to_string(&context)?;
                serde_json::from_str(&context)?
            }
            Some("json") => serde_json::from_str(&context)?,
            _ => unimplemented!(),
        };
        merge(&mut all_context, &context);
    }

    Ok(all_context)
}

// Source: https://stackoverflow.com/questions/47070876/how-can-i-merge-two-json-objects-with-rust
fn merge(a: &mut Value, b: &Value) {
    if let Value::Object(a) = a {
        if let Value::Object(b) = b {
            for (k, v) in b {
                if v.is_null() {
                    a.remove(k);
                } else {
                    merge(a.entry(k).or_insert(Value::Null), v);
                }
            }

            return;
        }
    }

    *a = b.to_owned();
}
