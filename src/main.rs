#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;
use itertools::Itertools;
use serde_json::Value;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let mut stdin = String::new();
    for line in io::stdin().lock().lines() {
        if let Ok(line) = line {
            stdin.push_str(&line)
        }
    }

    let data = serde_json::from_str::<Value>(&stdin)?;

    launch_cfg(
        app,
        LaunchConfig::builder()
            .with_title("Data visualizer")
            .with_width(700.0)
            .with_height(450.0)
            .with_state(data)
            .build(),
    );

    Ok(())
}

fn app(cx: Scope) -> Element {
    let data = cx.consume_context::<Value>().unwrap();

    // Gather the columns
    let mut columns = data
        .as_array()
        .unwrap_or(&Vec::default())
        .iter()
        .flat_map(|col| {
            if let Some(object) = col.as_object() {
                let keys = object
                    .keys()
                    .collect_vec()
                    .iter()
                    .map(|v| v.to_string())
                    .collect_vec();
                keys
            } else {
                Vec::default()
            }
        })
        .collect::<Vec<String>>();

    columns.sort();
    columns.dedup();

    // Gather the rows
    let rows = data
        .as_array()
        .unwrap_or(&Vec::default())
        .iter()
        .filter_map(|col| {
            if let Some(object) = col.as_object() {
                let mut properties = Vec::default();

                for column in columns.iter() {
                    let cell = object
                        .get(column)
                        .map(|prop| {
                            match prop {
                                Value::String(val) => Some(val.to_string()),
                                Value::Number(num) => Some(num.to_string()),
                                _ => None
                            }
                        })
                        .flatten()
                        .unwrap_or("❌".to_string());
                    properties.push(cell.to_string());
                }

                Some(properties)
            } else {
                None
            }
        })
        .collect::<Vec<Vec<String>>>();

    render!(
        rect {
            padding: "10",
            Table {
                columns: columns.len(),
                TableHead {
                    TableRow {
                        for (n, text) in columns.iter().enumerate() {
                            TableCell {
                                key: "{n}",
                                separator: n > 0,
                                label {
                                    width: "100%",
                                    align: "center",
                                    max_lines: "1",
                                    text_overflow: "ellipsis",
                                    "{text}"
                                }
                            }
                        }
                    }
                }
                TableBody {
                    ScrollView {
                        for (i, row) in rows.iter().enumerate() {
                            TableRow {
                                key: "{i}",
                                alternate_colors: i % 2 == 0,
                                for (n, item) in row.iter().enumerate() {
                                    TableCell {
                                        key: "{n}",
                                        separator: n > 0,
                                        label {
                                            width: "100%",
                                            align: "right",
                                            max_lines: "1",
                                            text_overflow: "ellipsis",
                                            "{item}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    )
}
