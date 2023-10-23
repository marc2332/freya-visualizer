#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;
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

const NUSHELL_THEME: Theme = Theme {
    table: TableTheme {
        color: "rgb(173, 186, 199)",
        background: "rgb(34, 39, 46)",
        arrow_fill: "rgb(240, 240, 240)",
        row_background: "transparent",
        alternate_row_background: "rgb(40, 44, 52)",
        divider_fill: "rgb(143, 156, 169)",
    },
    scrollbar: ScrollbarTheme {
        background: "rgb(44, 49, 56)",
        thumb_background: "rgb(74, 79, 86)",
        hover_thumb_background: "rgb(84, 89, 96)",
        active_thumb_background: "rgb(94, 99, 106)",
    },
    ..DARK_THEME
};

fn app(cx: Scope) -> Element {
    use_init_theme(cx, NUSHELL_THEME);
    let data = cx.consume_context::<Value>().unwrap();

    // Gather the columns
    let columns = {
        let mut columns = Vec::new();
        if let Some(rows) = data.as_array() {
            for row in rows {
                if let Some(object) = row.as_object() {
                    for col in object.keys() {
                        if !columns.contains(col) {
                            columns.push(col.to_owned());
                        }
                    }
                }
            }
        }
        columns
    };

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
                        .map(|prop| match prop {
                            Value::String(val) => Some(val.to_string()),
                            Value::Number(num) => Some(num.to_string()),
                            _ => None,
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
            background: "rgb(20, 24, 32)",
            Table {
                columns: columns.len(),
                TableHead {
                    TableRow {
                        for (n, text) in columns.iter().enumerate() {
                            TableCell {
                                key: "{n}",
                                divider: n > 0,
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
                                        divider: n > 0,
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
