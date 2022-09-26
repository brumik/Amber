use heraclitus_compiler::prelude::*;
use crate::modules::types::Type;
use crate::modules::variable::variable_name_extensions;
use crate::utils::metadata::{ParserMetadata, TranslateMetadata};
use crate::translate::module::TranslateModule;
use crate::modules::block::Block;
use crate::modules::types::parse_type;

use super::declaration_utils::*;

#[derive(Debug, Clone)]
pub struct FunctionDeclaration {
    pub name: String,
    pub args: Vec<(String, Type)>,
    pub returns: Type,
    pub body: Block,
    pub id: usize,
    pub is_public: bool
}

impl FunctionDeclaration {
    fn set_args_as_variables(&self, meta: &mut TranslateMetadata) -> Option<String> {
        if !self.args.is_empty() {
            meta.increase_indent();
            let mut result = vec![];
            for (index, (name, _kind)) in self.args.clone().iter().enumerate() {
                let indent = meta.gen_indent();
                result.push(format!("{indent}{name}=${}", index + 1));
            }
            meta.decrease_indent();
            Some(result.join("\n"))
        } else { None }
    }
}

impl SyntaxModule<ParserMetadata> for FunctionDeclaration {
    syntax_name!("Function Declaration");

    fn new() -> Self {
        FunctionDeclaration {
            name: String::new(),
            args: vec![],
            returns: Type::Generic,
            body: Block::new(),
            id: 0,
            is_public: false
        }
    }

    fn parse(&mut self, meta: &mut ParserMetadata) -> SyntaxResult {
        // Check if this function is public
        if token(meta, "pub").is_ok() {
            self.is_public = true;
        }
        token(meta, "fun")?;
        // Get the function name
        let tok = meta.get_current_token();
        self.name = variable(meta, variable_name_extensions())?;
        handle_existing_function(meta, tok.clone())?;
        return context!({
            // Get the arguments
            token(meta, "(")?;
            loop {
                if token(meta, ")").is_ok() {
                    break
                }
                let name = variable(meta, variable_name_extensions())?;
                // Optionally parse the argument type
                match token(meta, ":") {
                    Ok(_) => self.args.push((name.clone(), parse_type(meta)?)),
                    Err(_) => self.args.push((name, Type::Generic))
                }
                match token(meta, ")") {
                    Ok(_) => break,
                    Err(_) => token(meta, ",")?
                };
            }
            // Optionally parse the return type
            match token(meta, ":") {
                Ok(_) => self.returns = parse_type(meta)?,
                Err(_) => self.returns = Type::Text
            }
            // Parse the body
            token(meta, "{")?;
            let (index_begin, index_end) = skip_function_body(meta);
            token(meta, "}")?;
            // Add the function to the memory
            self.id = handle_add_function(meta, tok, FunctionDeclSyntax {
                name: self.name.clone(),
                args: self.args.clone(),
                returns: self.returns.clone(),
                body: meta.expr[index_begin..index_end].to_vec(),
                is_public: self.is_public
            })?;
            Ok(())
        }, |pos| {
            error_pos!(meta, pos, format!("Failed to parse function declaration '{}'", self.name))
        })
    }
}

impl TranslateModule for FunctionDeclaration {
    fn translate(&self, meta: &mut TranslateMetadata) -> String {
        let mut result = vec![];
        let blocks = meta.mem.get_function_instances(self.id).unwrap().to_vec();
        // Translate each one of them
        for (index, function) in blocks.iter().enumerate() {
            let mut name = self.name.clone();
            if index != 0 {
                name = format!("__{}_{}", index, name);
            }
            // Parse the function body
            result.push(format!("function {} {{", name));
            if let Some(args) = self.set_args_as_variables(meta) {
                result.push(args); 
            }
            result.push(function.body.translate(meta));
            result.push(meta.gen_indent() + "}");
        }
        // Return the translation
        result.join("\n")
    }
}