#[cfg(test)]
pub mod examples {
    pub mod common_concepts;
    pub mod macros;
    pub mod ownership;
    pub mod control_flow;
    pub mod prototype_structures {
        pub mod enums;
        pub mod structs;
        pub mod unions;
    }

    pub mod collections;

    pub mod modules;

    pub mod advanced {
        pub mod generics;
        pub mod traits;
        pub mod lifetime_specification;

        pub mod functional {
            pub mod closures;
            pub mod iterators;
        }
         
    }

    pub mod error_handling;
}

fn main() {
     
 }
