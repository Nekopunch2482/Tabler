use std::{fs::File, io::Write, process::Command};

pub fn convert_doc(files: Vec<(std::path::PathBuf, String)>) -> Vec<(std::path::PathBuf, String)> {
    files
        .into_iter()
        .map(|(path, name)| {
            let ext = match path.extension() {
                Some(ext) => ext,

                None => {
                    panic!(
                        "Could not extract file extension from path: {}",
                        path.display()
                    );
                }
            };

            if ext == "doc" {
                let input = path.to_str().unwrap();

                let exe_path = std::env::current_exe().unwrap();
                let exe_dir = exe_path.parent().unwrap();
                let output = exe_dir.join(&name).with_extension("docx");

                convert_doc_to_docx_windows(input, output.to_str().unwrap());

                (output, name)
            } else {
                (path, name)
            }
        })
        .collect::<Vec<(std::path::PathBuf, String)>>()
}

fn convert_doc_to_docx_windows(input: &str, output: &str) -> () {
    let ps_script = format!(
        r#"
        $word = New-Object -ComObject Word.Application
        $word.Visible = $false
        $doc = $word.Documents.Open("{}")
        $doc.SaveAs2("{}", 12)
        $doc.Close()
        $word.Quit()
        "#,
        input.replace("\\", "\\\\"),
        output.replace("\\", "\\\\")
    );

    let temp_dir = std::env::temp_dir();
    let temp_script_path = temp_dir.join("convert_doc.ps1");
    let mut temp_file = File::create(&temp_script_path).unwrap();
    // write BOM header
    temp_file.write_all(&[0xEF, 0xBB, 0xBF]).unwrap();
    // write rest of the script
    temp_file.write_all(ps_script.as_bytes()).unwrap();
    temp_file.flush().unwrap();

    // close file or powershell will complain
    drop(temp_file);

    let file_path_str = temp_script_path.to_string_lossy().into_owned();

    println!("->> converting file: {file_path_str}");

    let output = Command::new("powershell")
        .args(&[
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-File",
            &file_path_str,
        ])
        .output()
        .unwrap();

    match output.status.code() {
        Some(0) => {
            println!("->> converting file: {file_path_str} ...Ok");
        }
        _ => {
            println!("->> converting file: {file_path_str} ...Fail");

            std::io::stdout().write_all(&output.stdout).unwrap();
            std::io::stderr().write_all(&output.stderr).unwrap();
        }
    }

    ()
}
