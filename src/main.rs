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

fn get_functions_from_entity<'a>(entity: &Entity<'a>) -> Vec<Entity<'a>> {
    let mut functions: Vec<Entity> = vec![];
    for e in entity.get_children().iter() {
        match e.get_kind() {
            EntityKind::FunctionDecl     => { if e.is_definition() { functions.push(*e); } },
            EntityKind::FunctionTemplate => { if e.is_definition() { functions.push(*e); } },
            EntityKind::Method           => { if e.is_definition() { functions.push(*e); } },
            EntityKind::ClassDecl        => { if e.is_definition() { functions.extend(get_functions_from_entity(e)) } },
            EntityKind::ClassTemplate    => { if e.is_definition() { functions.extend(get_functions_from_entity(e)) } },
            EntityKind::ClassTemplatePartialSpecialization => { if e.is_definition() { functions.extend(get_functions_from_entity(e)) } },
            EntityKind::StructDecl       => { if e.is_definition() { functions.extend(get_functions_from_entity(e)) } },
            EntityKind::Namespace        => { if e.is_definition() { functions.extend(get_functions_from_entity(e)) } },
            _ => {},
        }
    }
    functions
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
        let entity = tu.get_entity();
        functions.extend(get_functions_from_entity(&entity));
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
