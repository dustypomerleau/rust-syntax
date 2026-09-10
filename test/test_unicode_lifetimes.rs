// SYNTAX TEST "source.rust" "Non-ASCII lifetime names"

    fn lifetime_param<'ä>() {}
//                    ^ punctuation.definition.lifetime.rust
//                     ^ entity.name.type.lifetime.rust

    fn borrowed(x: &'ö str) {}
//                   ^ entity.name.type.lifetime.rust
//                     ^^^ storage.type.primitive.rust
