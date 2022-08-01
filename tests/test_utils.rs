use averse::utils::*;
use std::path::Path;
use tabled::Tabled;

#[test]
fn test_table() {
    #[derive(Tabled)]
    struct Foo {
        Col1: String,
        Col2: String,
    }
    let foo = Foo {
        Col1: "Foo".to_string(),
        Col2: "Bar".to_string(),
    };
    print_table(&vec![foo]);
}

#[test]
fn test_recipe_path() {
    let dir = "Some/arbitrary/path".to_string();
    let name = "Recipe name".to_string();
    let path = get_recipe_out_path(&dir, &name);
    assert_eq!(
        "Some/arbitrary/path/Recipe-name.yaml",
        path.to_str().unwrap()
    );
}

#[test]
fn test_get_jsons() {
    let dir = Path::new("./recipes");
    let jsons = get_jsons(dir).unwrap();
    assert!(jsons.len() > 1);
    let ext = &jsons[0].extension().unwrap().to_str().unwrap();
    assert_eq!(*ext, "yaml");
}
