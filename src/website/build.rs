use cynic_codegen::register_schema;

fn main() {
    register_schema("schema")
        .from_sdl_file("src/schema.graphql")
        .unwrap()
        .as_default()
        .unwrap();
}
