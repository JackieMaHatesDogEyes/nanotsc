
use deno_ast::{parse_module, ParseParams, SourceTextInfo, MediaType, EmitOptions};
use std::{env, fs};

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    for f in args {
        let filename = f.clone();
        let file_contents = fs::read_to_string(f).expect(format!("Failed to open file: {}.", filename).as_str());

        let new_file = transpile(file_contents.as_str(), filename.as_str());
        
        let mut new_file_name = filename.strip_suffix(".ts").unwrap().to_owned();
        new_file_name = format!("{new_file_name}.js");

        fs::write(new_file_name, new_file).expect("Failed to create file.");
        
    }

}

pub fn transpile(code: &str, filename: &str) -> String {
    

    unsafe { 
        let parsed  = parse_module(ParseParams {
            specifier: filename.to_owned(),
            text_info: SourceTextInfo::from_string(code.to_owned()),
            media_type: MediaType::TypeScript,
            capture_tokens: false,
            scope_analysis: false,
            maybe_syntax: None,
        }).unwrap_unchecked();
    

        let mut opts = EmitOptions::default();
        opts.inline_source_map = false;
        


        let js = parsed.transpile(&opts).unwrap().text;

        return js;

    }
}