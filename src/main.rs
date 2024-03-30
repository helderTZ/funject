use clang::*;
use std::env;

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
                e.get_kind() == EntityKind::FunctionDecl ||
                e.get_kind() == EntityKind::FunctionTemplate ||
            }).collect::<Vec<_>>()
        );
    }
    //TODO: get classes and get their children to reach the method decls

    for function in functions.iter() {
        let location = function.get_location().unwrap().get_expansion_location();
        println!("function: {:?} @ {:?}:{}:{}:{}",
            function.get_name().unwrap(),
            location.file.unwrap().get_path(),
            location.line,
            location.column,
            location.offset
        );
    }
}
