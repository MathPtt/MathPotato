//! Abstract Syntax Tree public Api
//!
//! This Api is for the Parser and Interpreter to communicate with the AST.
//! The AST itself is a bit complicated inside, so a well managed, hopefully clear enough,
//! sometimes just pedantic Api was needed.
//!
//! # Modularisation
//! Every Api has its own submodule, and every Api method has its own sibling module. I saw this
//! necessary to keep the files short and easy to oversee. Moreover, the tests are attached to the
//! module and since I believe in the exhaustive testing they would have been made the files
//! enourmous. Nobody wants that.
pub mod cont_node_api;
pub mod i32_api;
pub mod infix_api;
pub mod root_node;
