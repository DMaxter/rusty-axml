pub mod cli;
pub mod parser;
pub mod xml_types;
pub mod chunk_header;
pub mod string_pool;
pub mod resource_map;
pub mod data_value_type;
pub mod res_value;
pub mod res_table;

use std::{
    fs,
    collections::HashMap,
};
use std::io::{
    Read,
    Cursor,
};
use std::rc::Rc;
use std::cell::RefCell;

use crate::chunk_header::ChunkHeader;
use crate::resource_map::ResourceMap;
use crate::res_table::{
    ResTable
};
use crate::string_pool::StringPool;
use crate::xml_types::XmlTypes;
use crate::parser::XmlElement;

/// Representation of an app's manifest contents
#[derive(Debug, Default)]
pub struct ManifestContents {
    pub pkg_name: String,

    pub activities: Vec<String>,
    pub services: Vec<String>,
    pub providers: Vec<String>,
    pub receivers: Vec<String>,

    // TODO: does not includes permissions requested from within components
    pub created_perms: Vec<String>,
    pub requested_perms: Vec<String>,

    pub main_entry_point: Option<String>,
}

/// A component can be exported or enabled. Each of these feature have default values
/// but these default values can be overriden by the developer. This means they have
/// essentially four states:
///     * default to `true`,
///     * default to `false`,
///     * explicitely set to `true`,
///     * explicitely set to `false`
#[derive(Debug, PartialEq)]
pub enum ComponentState {
    Unknown,
    DefaultTrue,
    DefaultFalse,
    ExplicitTrue,
    ExplicitFalse,
}

/// Open the file, read the contents, and create a `Cursor` of the raw data
/// for easier handling when parsing the XML data.
pub fn create_cursor(file_path: &str, arg_type: cli::ArgType) -> Cursor<Vec<u8>> {

    let mut axml_cursor = Vec::new();

    if arg_type == cli::ArgType::Apk {
        // If we are dealing with an APK, we must first extract the binary XML from it
        // In this case we assume the user wants to decode the app manifest so we extract that

        let zipfile = std::fs::File::open(file_path).unwrap();
        let mut archive = zip::ZipArchive::new(zipfile).unwrap();
        let mut raw_file = match archive.by_name("AndroidManifest.xml") {
            Ok(file) => file,
            Err(..) => {
                panic!("Error: no AndroidManifest.xml in APK");
            }
        };
        raw_file.read_to_end(&mut axml_cursor).expect("Error: cannot read manifest from app");
    } else {
        let mut raw_file = fs::File::open(file_path).expect("Error: cannot open AXML file");
        raw_file.read_to_end(&mut axml_cursor).expect("Error: cannot read AXML file");
    }

    Cursor::new(axml_cursor)
}

pub fn get_manifest_contents(axml_cursor: Cursor<Vec<u8>>) -> Rc<RefCell<XmlElement>> {
    parser::parse_xml(axml_cursor)
}

/// Parse an app's manifest and extract interesting contents
/// For now, only these elements are extracted, although that
/// list might get longer in the future:
///
///   * package name
///   * list of activities names
///   * list of services names
///   * list of content providers names
///   * list of broadcast receiver names
fn REAL_get_manifest_contents(mut axml_cursor: Cursor<Vec<u8>>) -> ManifestContents {
    let mut contents = ManifestContents::default();

    let mut global_strings = Vec::new();
    let mut namespace_prefixes = HashMap::<String, String>::new();
    // let mut writer = Vec::new();

    loop {
        if let Ok(block_type) = XmlTypes::parse_block_type(&mut axml_cursor) {
            match block_type {
                XmlTypes::ResNullType => continue,
                XmlTypes::ResStringPoolType => {
                    let _ = StringPool::from_buff(&mut axml_cursor, &mut global_strings);
                },
                XmlTypes::ResTableType => {
                    let _ = ResTable::parse(&mut axml_cursor);
                },
                XmlTypes::ResXmlType => {
                    axml_cursor.set_position(axml_cursor.position() - 2);
                    let _ = ChunkHeader::from_buff(&mut axml_cursor, XmlTypes::ResXmlType);
                },
                XmlTypes::ResXmlStartNamespaceType => {
                    parser::parse_start_namespace(&mut axml_cursor, &global_strings, &mut namespace_prefixes);
                },
                XmlTypes::ResXmlEndNamespaceType => {
                    parser::parse_end_namespace(&mut axml_cursor, &global_strings);
                },
                XmlTypes::ResXmlStartElementType => {
                    let (element_type, attrs) = parser::parse_start_element(&mut axml_cursor, &global_strings, &namespace_prefixes).unwrap();

                    // Get element name from the attributes
                    // We only care about package name, activites, services, content providers and
                    // broadcast receivers which all have their name in the "android" namespace
                    let mut element_name = String::new();

                    for (attr_key, attr_val) in attrs.iter() {
                        if attr_key == "android:name" {
                            element_name = attr_val.to_string();
                            break;
                        }
                    }

                    match element_type.as_str() {
                        "activity" => contents.activities.push(element_name),
                        "service"  => contents.services.push(element_name),
                        "provider" => contents.providers.push(element_name),
                        "receiver" => contents.receivers.push(element_name),
                        "permission" => contents.created_perms.push(element_name),
                        "uses-permission" => contents.requested_perms.push(element_name),
                        "action" if element_name == "android.intent.action.MAIN" => contents.main_entry_point = contents.activities.last().cloned(),
                        _ => { }
                    }

                    // Package name is in the "manifest" element and with the "package" key
                    if element_type == "manifest" {
                        for (attr_key, attr_val) in attrs.iter() {
                            if attr_key == "package" {
                                contents.pkg_name = attr_val.to_string();
                                break;
                            }
                        }
                    }
                },
                XmlTypes::ResXmlEndElementType => {
                    parser::parse_end_element(&mut axml_cursor, &global_strings).unwrap();
                },

                XmlTypes::ResXmlResourceMapType => {
                    let _ = ResourceMap::from_buff(&mut axml_cursor);
                },

                _ => { },
            }
        }
        else  {
            break;
        }
    }

    contents
}

/// Convenience function to parse the manifest of an APK
pub fn parse_app_manifest(file_path: &str) -> Cursor<Vec<u8>> {
    create_cursor(file_path, cli::ArgType::Apk)
}

/// Use BFS tree traversal to get all element of a given type
fn find_elements_by_type(parsed_xml: &Rc<RefCell<XmlElement>>, element_type: &str) -> Vec<Rc<RefCell<XmlElement>>> {
    let mut result = Vec::new();
    let mut stack = vec![Rc::clone(parsed_xml)];

    while let Some(element) = stack.pop() {
        let borrowed = element.borrow();
        if borrowed.element_type == element_type {
            result.push(Rc::clone(&element));
        }
        for child in &borrowed.children {
            stack.push(Rc::clone(child));
        }
    }

    result
}

/// Check if a component is exposed which is the case if it is both enabled and exported
/// Both of these properties can either be explicitely set (as parameters in the compoennt
/// declaration in the manifest) or left to their default state.
/// The default state depends on the presence or not of intent filters: if there is an intent
/// filter, the assumption is that the compoennt is meants to be available to other apps, and so it
/// is exported by default, otherwise not.
fn is_component_exposed(component: &Rc<RefCell<XmlElement>>) -> bool {
    let mut enabled_state = ComponentState::DefaultTrue;
    let mut exported_state = ComponentState::Unknown;

    if let Some(enabled) = component.borrow().attributes.get("android:enabled") {
        if enabled == "false" {
            return false;
        } else {
            enabled_state = ComponentState::ExplicitTrue;
        }
    }

    if let Some(exported) = component.borrow().attributes.get("android:exported") {
        if exported == "false" {
            return false;
        } else {
            exported_state = ComponentState::ExplicitTrue;
        }
    }

    // If the component has intent filters then the default exported value is `true`, otherwise
    // `false`. This is not the case for content providers though, which usually have explicit
    // values anyway.
    if exported_state == ComponentState::Unknown {
        for item in component.borrow().children.iter() {
            if item.borrow().element_type == "intent-filter" {
                exported_state = ComponentState::DefaultTrue;
                break;
            }
        }
        if exported_state == ComponentState::Unknown {
            exported_state = ComponentState::DefaultFalse;
        }
    }

    // At this point we know the component is enabled so we just need to check if it is also
    // exported. Also, if the component is explicitly not exported then we return early so here we
    // do not have to check all the cases
    match exported_state {
        ComponentState::DefaultFalse => false,
        ComponentState::DefaultTrue => true,
        ComponentState::ExplicitTrue => true,
        _ => panic!("never going to happen")
    }
}

/// Parse an app's manifest and get the list of exposed components
/// We first check if the app has the `android:enabled` component set, which would influence the
/// state of all the components in the app
pub fn get_exposed_components(parsed_xml: Rc<RefCell<XmlElement>>) -> Option<HashMap<String, Vec<Rc<RefCell<XmlElement>>>>> {
    // Checking if the `<application>` tag has the `enabled` attribute set to `false`
    let application = find_elements_by_type(&parsed_xml, "application").pop()?;
    if let Some(enabled) = application.borrow().attributes.get("android:enabled") {
        if enabled == "false" {
            return None;
        }
    }

    let mut components = HashMap::new();

    components.insert(
        String::from("activity"),
        find_elements_by_type(&parsed_xml, "activity")
                .into_iter()
                .filter(|item| is_component_exposed(&item))
                .collect()
    );
    components.insert(
        String::from("service"),
        find_elements_by_type(&parsed_xml, "service")
                .into_iter()
                .filter(|item| is_component_exposed(&item))
                .collect()
    );
    components.insert(
        String::from("provider"),
        find_elements_by_type(&parsed_xml, "provider")
                .into_iter()
                .filter(|item| is_component_exposed(&item))
                .collect()
    );
    components.insert(
        String::from("receiver"),
        find_elements_by_type(&parsed_xml, "receiver")
                .into_iter()
                .filter(|item| is_component_exposed(&item))
                .collect()
    );

    Some(components)
}

