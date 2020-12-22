
use serde::{Deserialize, Serialize};
use serde_json::{Result, value::RawValue};

#[derive(Deserialize)]
struct Input<'a> {
    code: u32,
    #[serde(borrow)]
    payload: &'a RawValue,
}

#[derive(Serialize)]
struct Output<'a> {
    info: (u32, &'a RawValue),
}

// Efficiently rearrange JSON input containing separate "code" and "payload"
// keys into a single "info" key holding an array of code and payload.
//
// This could be done equivalently using serde_json::Value as the type for
// payload, but &RawValue will perform better because it does not require
// memory allocation. The correct range of bytes is borrowed from the input
// data and pasted verbatim into the output.
fn rearrange(input: &str) -> Result<String> {
    let input: Input = serde_json::from_str(input)?;

    let output = Output {
        info: (input.code, input.payload),
    };

    assert_eq!(serde_json::to_string(&input.payload)?, r#"{}"#);

    serde_json::to_string(&output)
}

fn main() -> Result<()> {
    
    let out = rearrange(r#" {"code": 200, "payload": {}} "#)?;

    assert_eq!(out, r#"{"info":[200,{}]}"#);

    Ok(())
}