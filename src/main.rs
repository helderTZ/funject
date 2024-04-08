use clang::*;
use std::{env, fs};

fn get_next_left_bracket(string: &str, idx: usize) -> usize {
    let mut idx_of_next_left_bracket = idx;
    for i in idx..string.len() {
        if string.chars().nth(i).unwrap() == '{' {
            return idx_of_next_left_bracket+2;
        }
        idx_of_next_left_bracket += 1;
    }
    idx_of_next_left_bracket
}

fn is_entity_from_files(entity: &Entity, files: &[String]) -> bool {
    let file = entity.get_location().unwrap().get_file_location().file.unwrap().get_path();
    files.contains(&file.into_os_string().into_string().unwrap())
}

fn get_functions_from_entity<'a>(entity: &Entity<'a>, follow_inc: bool, files: &[String]) -> Vec<Entity<'a>> {
    let mut functions: Vec<Entity> = vec![];
    for e in entity.get_children().iter() {
        if follow_inc || is_entity_from_files(e, files) {
            match e.get_kind() {
                EntityKind::FunctionDecl     => { if e.is_definition() { functions.push(*e); } },
                EntityKind::FunctionTemplate => { if e.is_definition() { functions.push(*e); } },
                EntityKind::Method           => { if e.is_definition() { functions.push(*e); } },
                EntityKind::ClassDecl        => { if e.is_definition() { functions.extend(get_functions_from_entity(e, follow_inc, files)) } },
                EntityKind::ClassTemplate    => { if e.is_definition() { functions.extend(get_functions_from_entity(e, follow_inc, files)) } },
                EntityKind::ClassTemplatePartialSpecialization => { if e.is_definition() { functions.extend(get_functions_from_entity(e, follow_inc, files)) } },
                EntityKind::StructDecl       => { if e.is_definition() { functions.extend(get_functions_from_entity(e, follow_inc, files)) } },
                EntityKind::Namespace        => { if e.is_definition() { functions.extend(get_functions_from_entity(e, follow_inc, files)) } },
                _ => {},
            }
        }
    }
    functions
}

// Adapted from: https://stackoverflow.com/a/76820878/13499951
fn get_files_in_dir(path: String) ->Vec<String> {
    let Ok(entries) = fs::read_dir(path) else { return vec![] };
    entries.flatten().flat_map(|e| {
        let Ok(meta) = e.metadata() else { return vec![] };
        if meta.is_dir() {
            return get_files_in_dir(e.path().display().to_string());
        }
        if meta.is_file() && (e.path().extension().unwrap_or_default() == "cpp"
            || e.path().extension().unwrap_or_default() == "cc"
            || e.path().extension().unwrap_or_default() == "c"
            || e.path().extension().unwrap_or_default() == "hpp"
            || e.path().extension().unwrap_or_default() == "h"
        ) {
            return vec![e.path().as_path().display().to_string()];
        }
        vec![]
    }).collect()
}

fn usage() {
    println!("Usage: funject [OPTIONS] [file1] [file2] ...");
    println!("Options:");
    println!("    --help, -h : Print this help.");
    println!("    --inject   : Perform code injection (dry run if not supplied).");
    println!("    --quiet    : Disables logging of the functions found.");
    println!("    --dir <dirname> : Searches recursively for C/C++ source files in the directory `dirname`");
    println!("                      Overwrites files given as arguments.");
    println!("                      Searches only for files with extension *.c, *.cc, *.cpp, *.h, *.hpp .");
    println!("    --follow-inc    : Recursively parse down '#include' directives and continue parsing the included file");
}

fn main() {
    let commands = ["--inject", "--quiet", "--dir", "--help", "-h", "--follow-inc"];

    let args: Vec<String> = env::args().skip(1).collect();
    let help: bool = args.contains(&"--help".to_owned()) || args.contains(&"-h".to_owned());
    if help {
        usage();
        return;
    }

    let inject: bool = args.contains(&"--inject".to_owned());
    let quiet: bool = args.contains(&"--quiet".to_owned());
    let follow_inc: bool = args.contains(&"--follow-inc".to_owned());
    let dir: bool = args.contains(&"--dir".to_owned());
    let mut dirname: Option<String> = None;
    if dir {
        let idx = args.iter().position(|a| { a == "--dir"}).unwrap()+1;
        dirname = Some(args[idx].clone());
    }
    let files = match dirname {
        Some(dirname) => get_files_in_dir(dirname),
        None => args.into_iter().filter(|a| { !commands.contains(&a.as_str()) }).collect::<Vec<String>>(),
    };
    println!("found {} files.", files.len());

    let clang = Clang::new().unwrap();
    let index = Index::new(&clang, false, false);

    let mut translation_units: Vec<TranslationUnit> = vec![];
    for file in files.iter() {
        if let Ok(tu) = index.parser(file).parse() {
            translation_units.push(tu);
        }
    }
    println!("Parsed {} translation units.", translation_units.len());

    let mut functions: Vec<Entity> = vec![];
    for tu in translation_units.iter() {
        let entity = tu.get_entity();
        functions.extend(get_functions_from_entity(&entity, follow_inc, &files));
    }
    println!("Found {} function definitions.", functions.len());

    if !quiet {
        for function in functions.iter() {
            let location = function.get_location().unwrap().get_expansion_location();
            println!("function: {} @ {}:{}:{}:{}",
                function.get_name().unwrap(),
                location.file.unwrap().get_path().as_path().display(),
                location.line,
                location.column,
                location.offset
            );
        }
    }

    if inject {
        let injection = "    printf(\"Injected!\\n\");\n";
        let mut updated_offset: usize = 0;
        for function in functions.iter() {
            let location = function.get_location().unwrap().get_expansion_location();
            let mut file_str = fs::read_to_string(location.file.unwrap().get_path()).unwrap();
            let offset: usize = get_next_left_bracket(&file_str, location.offset as usize + updated_offset);
            file_str.insert_str(offset, injection);
            fs::write(location.file.unwrap().get_path(), file_str).unwrap();
            updated_offset += injection.len();
        }
    }

}
