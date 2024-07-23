#![allow(non_snake_case, unused_variables, dead_code)]

use std::{
    fs,
    collections::HashMap,
};
use std::io::{
    Write,
    Cursor,
};

use quick_xml::Writer;

use axml_parser::create_cursor;
use axml_parser::chunk_header::ChunkHeader;
use axml_parser::resource_map::ResourceMap;
use axml_parser::res_table::{
    ResTable,
    ResTablePackage
};
use axml_parser::string_pool::StringPool;
use axml_parser::xml_types::XmlTypes;
use axml_parser::parser;
use axml_parser::cli;

fn main() {
    // Check CLI arguments
    let args = cli::parse_args();

    // Check the file type
    let arg_type = args.get_arg_type();
    let arg_path = args.get_arg_path();

    // Create cursor over input file contents
    let mut axml_cursor = create_cursor(&arg_path, arg_type);

    let elements = parser::parse_xml(axml_cursor);
    println!("{elements:?}");

    /*
    let result = writer.into_inner().into_inner();
    let str_result = String::from_utf8(result).unwrap();

    if args.output.is_some() {
        let mut file = fs::File::create(&args.output.unwrap()).unwrap();
        file.write_all(str_result.as_bytes()).unwrap();
    } else {
        println!("{str_result}");
    }
    */

}
