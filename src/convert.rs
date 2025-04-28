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

                convert_doc_to_docx_windows(input, output.to_str().unwrap()).unwrap();

                (output, name)
            } else {
                (path, name)
            }
        })
        .collect::<Vec<(std::path::PathBuf, String)>>()
}

#[cfg(windows)]
fn convert_doc_to_docx_windows(input: &str, output: &str) -> windows::core::Result<()> {
    use windows::{
        core::*,
        Win32::System::Com::{
            CoCreateInstance, CoInitializeEx, CoUninitialize, IDispatch, CLSCTX_ALL,
            COINIT_APARTMENTTHREADED,
        },
        Win32::System::Ole::CLSIDFromProgID,
        Win32::System::Variant::VARIANT,
    };

    unsafe {
        CoInitializeEx(std::ptr::null_mut(), COINIT_APARTMENTTHREADED)?;

        let word: IDispatch =
            CoCreateInstance(&CLSIDFromProgID("Word.Application")?, None, CLSCTX_ALL)?;

        let _ = word.SetProperty("Visible", VARIANT::from(false));

        let documents = word.GetProperty("Documents")?.to_dispatch()?;
        let doc = documents
            .Invoke("Open", &[VARIANT::from(input)])?
            .to_dispatch()?;

        doc.Invoke("SaveAs", &[VARIANT::from(output), VARIANT::from(16)])?;

        let _ = doc.Invoke("Close", &[]);
        let _ = word.Invoke("Quit", &[]);

        CoUninitialize();
    }

    Ok(())
}

#[cfg(not(windows))]
fn convert_doc_to_docx_windows(_input: &str, _output: &str) -> Result<(), &'static str> {
    Err("Word automation is only supported on Windows")
}
