use serde::Serialize;
use streaming_iterator::StreamingIterator;
use tree_sitter::{Language, Parser as TSParser, Query, QueryCursor};

//TODO: achar lugar melhor pra colocar essas structs
#[derive(Serialize, Debug)]
pub struct FuncInfo {
    pub comment: String,
    pub signature: String,
}

#[derive(Serialize, Debug)]
pub struct FieldInfo {
    pub name: String,
    pub data_type: String,
}

#[derive(Serialize, Debug)]
pub struct StructInfo {
    pub name: String,
}

#[derive(Serialize, Debug)]
pub struct TemplateData {
    pub name: String,
    pub source_code: String,
    pub structs: Vec<StructInfo>,
    pub fields: Vec<FieldInfo>,
    pub functions: Vec<FuncInfo>,
}

pub fn extract_ast(
    source_code: &str,
) -> Result<(Vec<StructInfo>, Vec<FieldInfo>, Vec<FuncInfo>), Box<dyn std::error::Error>> {
    let query_str = r#"
        (struct_item name: (type_identifier) @struct.name)
        (field_declaration name: (field_identifier) @field.name) 
        (function_item name: (identifier) @func.name)
    "#;

    let mut parser = TSParser::new();

    let language: Language = tree_sitter_rust::LANGUAGE.into();

    parser.set_language(&language)?;

    let query = Query::new(&language, query_str)?;

    let tree = parser
        .parse(source_code, None)
        .unwrap_or_else(|| panic!("Falha ao analisar o código-fonte"));

    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(&query, tree.root_node(), source_code.as_bytes());

    let mut structs = Vec::new();
    let mut fields = Vec::new();
    let mut functions = Vec::new();

    while let Some(m) = matches.next() {
        for capture in m.captures {
            let node = capture.node;
            if let Ok(text) = node.utf8_text(source_code.as_bytes()) {
                let text = text.to_string();
                let capture_name = query.capture_names()[capture.index as usize];
                match capture_name {
                    "struct.name" => structs.push(StructInfo { name: text }),
                    "field.name" => fields.push(FieldInfo {
                        name: text.clone(),
                        data_type: "unknown".to_string(),
                    }),
                    "func.name" => {
                        functions.push(FuncInfo {
                            comment: "Comentário autogerado".to_string(),
                            signature: node.parent().map_or(text.clone(), |p| {
                                p.utf8_text(source_code.as_bytes())
                                    .unwrap_or(&text)
                                    .lines()
                                    .next()
                                    .unwrap_or(&text)
                                    .to_string()
                            }),
                        });
                    }
                    _ => {}
                }
            }
        }
    }
    return Ok((structs, fields, functions));
}
