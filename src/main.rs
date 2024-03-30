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
    return idx_of_next_left_bracket;
}

fn main() {
    let clang = Clang::new().unwrap();
    let index = Index::new(&clang, false, false);

    let files: Vec<String> = env::args().skip(1).collect();
    let mut translation_units: Vec<TranslationUnit> = vec![];
    for file in files {
        translation_units.push(
            index.parser(file).parse().unwrap()
        );
    }

    let mut functions: Vec<Entity> = vec![];

    for tu in translation_units.iter() {
        functions.extend(
            tu.get_entity().get_children().into_iter().filter(|e| {
                (e.get_kind() == EntityKind::FunctionDecl ||
                 e.get_kind() == EntityKind::FunctionTemplate)
                && e.is_definition()
            }).collect::<Vec<_>>()
        );
    }

    for tu in translation_units.iter() {
        let classes = tu.get_entity().get_children().into_iter().filter(|e| {
            e.get_kind() == EntityKind::ClassDecl ||
            e.get_kind() == EntityKind::StructDecl ||
            e.get_kind() == EntityKind::ClassTemplate ||
            e.get_kind() == EntityKind::ClassTemplatePartialSpecialization
        }).collect::<Vec<_>>();
        for klass in classes.iter() {
            functions.extend(
                klass.get_children().into_iter().filter(|e| {
                    e.get_kind() == EntityKind::Method
                    && e.is_definition()
                }).collect::<Vec<_>>()
            );
        }
    }

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
