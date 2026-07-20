use serde::Serialize;
use streaming_iterator::StreamingIterator;
use tree_sitter::{Language, Parser as TSParser, Query, QueryCursor};

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
    pub path: String,
}

pub fn extract_ast(
    source_code: &str,
) -> Result<(Vec<StructInfo>, Vec<FieldInfo>, Vec<FuncInfo>), Box<dyn std::error::Error>> {
    let mut parser = TSParser::new();
    let language: Language = tree_sitter_rust::LANGUAGE.into();
    parser.set_language(&language)?;

    let query_str = r#"
    (struct_item 
        name: (type_identifier) @struct.name)
    (field_declaration 
        name: (field_identifier) @field.name 
        type: (_) @field.type) 
    (function_item) @func.full"#;

    let query = Query::new(&language, query_str)?;
    let tree = parser.parse(source_code, None).ok_or("Falha ao analisar")?;
    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(&query, tree.root_node(), source_code.as_bytes());

    let mut structs = Vec::new();
    let mut fields = Vec::new();
    let mut functions = Vec::new();

    while let Some(m) = matches.next() {
        for capture in m.captures {
            let name = query.capture_names()[capture.index as usize];
            let node = capture.node;
            let text = node.utf8_text(source_code.as_bytes()).unwrap_or("");

            match name {
                "struct.name" => structs.push(StructInfo {
                    name: text.to_string(),
                }),

                "field.name" => {
                    let field_type = node
                        .next_sibling()
                        .and_then(|n| n.next_sibling())
                        .and_then(|n| n.utf8_text(source_code.as_bytes()).ok())
                        .unwrap_or("unknown");

                    fields.push(FieldInfo {
                        name: text.to_string(),
                        data_type: field_type.to_string(),
                    });
                }

                "func.full" => {
                    let full_text = text;
                    let signature = full_text
                        .split('{')
                        .next()
                        .unwrap_or(full_text)
                        .trim()
                        .to_string();

                    let prev = node.prev_sibling();
                    let mut comment = String::new();

                    if let Some(p) = prev {
                        if p.kind() == "line_comment" {
                            comment = p
                                .utf8_text(source_code.as_bytes())
                                .unwrap_or("")
                                .replace("//", "")
                                .trim()
                                .to_string();
                        }
                    }

                    functions.push(FuncInfo { comment, signature });
                }
                _ => {}
            }
        }
    }
    Ok((structs, fields, functions))
}
