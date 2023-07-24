use markdown::Block;

pub fn extract_code(source: &str) -> String {
    let ast = markdown::tokenize(source);
    let mut code = Vec::new();

    for token in ast {
        if let Block::CodeBlock(_lang, code_block) = token {
            code.push(code_block);
        }
    }

    code.join("\n")
}
