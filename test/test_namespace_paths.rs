// SYNTAX TEST "source.rust" "Namespace paths in parameterized types"

    Box<std::error::Error>;
//      ^^^ entity.name.namespace.rust
//           ^^^^^ entity.name.namespace.rust

    Box<dyn std::error::Error>;
//          ^^^ entity.name.namespace.rust
//               ^^^^^ entity.name.namespace.rust

    Outer<Inner<crate_name::module::Type>>;
//              ^^^^^^^^^^ entity.name.namespace.rust
//                          ^^^^^^ entity.name.namespace.rust
