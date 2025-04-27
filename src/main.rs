use docx_rs::{read_docx, TableChild, TableRowChild};
use rust_xlsxwriter::Workbook;

use std::env;
use std::io::{self, Read};
use std::path::Path;

fn main() {
    let mut files: Vec<(std::path::PathBuf, String)> = Vec::new();

    for arg in env::args().skip(1) {
        let path = Path::new(&arg);

        match path.extension() {
            Some(ext) => {
                if ext != "docx" && ext != "doc" {
                    panic!("Invalid file extension: {}", ext.to_str().unwrap());
                }
            }

            None => {
                panic!(
                    "Could not extract file extension from path: {}",
                    path.display()
                );
            }
        };

        match path.file_stem() {
            Some(file_stem) => {
                files.push((path.to_path_buf(), file_stem.to_str().unwrap().to_string()));
            }
            None => {
                panic!("Could not extract file name from path: {}", path.display());
            }
        }
    }

    files.iter().for_each(|(in_file, out_file)| {
        println!("Parsing file: {}", in_file.display());
        extract_table_from_docx(in_file, out_file);
    });

    println!("Press Enter to exit...");
    let _ = io::stdin().read(&mut [0u8]).unwrap();
}

fn extract_row_paragraphs(table_child: &TableChild) -> Vec<String> {
    let texts = match table_child {
        TableChild::TableRow(row) => row.cells.iter(),
    }
    .map(|cell| match cell {
        TableRowChild::TableCell(table_cell) => table_cell.children.clone(),
    })
    .flatten()
    .map(|cell_content| {
        match cell_content {
            docx_rs::TableCellContent::Paragraph(para) => para.children,
            _ => panic!("Direct cell child is not a paragraph"),
        }
        .iter()
        .map(|para_child| match para_child {
            docx_rs::ParagraphChild::Run(run) => run.children.clone(),
            _ => panic!("Invalid run type"),
        })
        .map(|run_childs| {
            run_childs
                .into_iter()
                .map(|run_child| match run_child {
                    docx_rs::RunChild::Text(text) => text.text,
                    docx_rs::RunChild::Sym(sym) => sym.char.clone(),
                    _ => panic!("Invalid text type"),
                })
                .collect::<Vec<String>>()
                .join("")
        })
        .collect::<Vec<String>>()
        .join("")
    })
    .collect();

    return texts;
}

fn extract_table_from_docx(in_file: &Path, out_file: &str) -> () {
    let docx_data = std::fs::read(in_file).expect("Failed to read docx");
    let docx = read_docx(&docx_data).expect("Failed to parse docx");

    let tables: Vec<_> = docx
        .document
        .children
        .iter()
        .filter_map(|c| match c {
            docx_rs::DocumentChild::Table(table) => Some(table),
            _ => None,
        })
        .collect();

    if tables.len() == 0 {
        panic!("no tables found");
    } else {
        println!("Found {} tables", tables.len());
    }

    let tables: Vec<_> = tables
        .iter()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(_, table)| table)
        .collect();

    let table_data: Vec<Vec<Vec<String>>> = tables
        .iter()
        .map(|table| {
            let mut t = table
                .rows
                .clone()
                .into_iter()
                .map(|row| extract_row_paragraphs(&row))
                .filter(|row| return row[1].len() > 0)
                .collect::<Vec<Vec<String>>>();
            t.drain(0..1);
            t
        })
        .collect();

    create_excel_table(&table_data, out_file).expect("Failed to create Excel table");
}
use rust_xlsxwriter::FormatAlign;
use rust_xlsxwriter::{Format, XlsxError};

fn create_excel_table(data: &Vec<Vec<Vec<String>>>, out_file: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    let mut row_num = 2;
    for r in data.iter() {
        for (row_idx, row) in r.iter().enumerate() {
            for (col_idx, cell) in row.iter().enumerate() {
                if col_idx == 0 {
                    worksheet.write_number((row_num) as u32, col_idx as u16, row_num - 1)?;
                } else {
                    worksheet.write_string((row_num) as u32, col_idx as u16, cell)?;
                }
            }
            row_num += 1;
        }
    }

    let format_default = Format::new()
        .set_align(FormatAlign::Center)
        .set_align(FormatAlign::VerticalCenter)
        .set_text_wrap();

    worksheet.set_row_height(0, to_h("0.42″"))?;
    worksheet.set_row_height(1, to_h("0.63″"))?;
    worksheet.set_column_width(0, to_w("0.83″"))?; // Номер

    worksheet.write_string_with_format(1, 1, "Наименование", &format_default)?;
    worksheet.set_column_width(1, to_w("2.61″"))?;

    worksheet.write_string_with_format(
        1,
        2,
        "Обозначение документа на поставку",
        &format_default,
    )?;
    worksheet.set_column_width(2, to_w("2.00″"))?;

    worksheet.write_string_with_format(1, 3, "Куда входит (обозначение)", &format_default)?;
    worksheet.set_column_width(3, to_w("1.72″"))?;

    worksheet.write_string_with_format(1, 4, "на изделие", &format_default)?;
    worksheet.set_column_width(4, to_w("1.07″"))?;

    worksheet.write_string_with_format(1, 5, "в комплекты", &format_default)?;
    worksheet.set_column_width(5, to_w("1.33″"))?;

    worksheet.write_string_with_format(1, 6, "на регулир.", &format_default)?;
    worksheet.set_column_width(6, to_w("1.07″"))?;

    worksheet.write_string_with_format(1, 7, "всего", &format_default)?;
    worksheet.set_column_width(7, to_w("0.60″"))?;

    worksheet.write_string_with_format(1, 8, "примечание", &format_default)?;
    worksheet.set_column_width(8, to_w("0.85″"))?;

    worksheet.write_string_with_format(1, 9, "Цена", &format_default)?;
    worksheet.set_column_width(9, to_w("0.83″"))?;

    worksheet.write_string_with_format(1, 10, "Стоимость", &format_default)?;
    worksheet.set_column_width(10, to_w("1.13″"))?;

    worksheet.write_string_with_format(1, 11, "номер счета", &format_default)?;
    worksheet.set_column_width(11, to_w("0.83″"))?;

    // MERGED CELLS
    worksheet.merge_range(0, 0, 1, 0, "№", &format_default)?;

    worksheet.merge_range(0, 4, 0, 6, "Количество", &format_default)?;

    workbook.save(format!("{out_file}.xlsx"))?;
    Ok(())
}

fn to_h(input: &str) -> f64 {
    let cleaned = input.trim().trim_end_matches('″');
    cleaned
        .parse::<f64>()
        .ok()
        .map(|inches| inches * 72.0)
        .expect("Failed to parse inches")
}

fn to_w(input: &str) -> f64 {
    let cleaned = input.trim().trim_end_matches('″');
    cleaned
        .parse::<f64>()
        .ok()
        .map(|inches| inches * 9.7)
        .expect("Failed to parse inches")
}

fn print_table(table: Vec<Vec<String>>) {
    let col_widths: Vec<usize> = vec![2, 30, 1, 20, 3, 1, 5, 5, 1];

    table.iter().for_each(|row| {
        row.iter().enumerate().for_each(|(i, text)| {
            print!(" {:<width$} |", text, width = col_widths[i]);
        });
        println!();
    });
}
