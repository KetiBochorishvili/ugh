use serde::Serialize;
use std::fs::File;
use std::io::BufWriter;
use std::error::Error;
use crate::graphml_parser::parse_graphml;
use crate::vt_parser::parse_vt_json;
use crate::input_type::is_file;
use crate::types::{Graph};
use crate::iops::read_file_to_string;
use crate::vt_api::VTClient;


pub async fn transform_graph(input: &str, mode: &str) -> Result<(), Box<dyn Error>> {
    let (nodes, edges) = if mode == "auto" {
        if is_file(input) {
            if input.ends_with(".graphml") || input.ends_with(".maltego") {
                parse_graphml(input)?
            } else if input.ends_with(".json") {
                let json_data = read_file_to_string(input)?;
                // let data = serde_json::to_string(&json_data)?;
                parse_vt_json(&json_data)?
            } else {
                return Err("Auto could not identify".into());
            }
        } else {
            return Err("Not Detected".into())
        }
    } else if mode == "graphml" || mode == "maltego" {
        parse_graphml(input)?
    } else if mode == "vtjson" {
        parse_vt_json(input)?
    } else if mode == "vtapi" {
        let client = VTClient::new();
        let vt_data = client.call_vt_hal(&input, None).await?;
        let json_data = serde_json::to_value(&vt_data)?;
        let data = serde_json::to_string(&json_data)?;
        parse_vt_json(&data)?
    } else {
        return Err("Unsupported mode".into());
    };

    let rich_graph = Graph { nodes, edges };

    save_json(&rich_graph, "rich.json")?;

    Ok(())
}

fn save_json<T: Serialize>(data: &T, filename: &str) -> Result<(), Box<dyn Error>> {
    let file = File::create(filename)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, data)?;
    Ok(())
}