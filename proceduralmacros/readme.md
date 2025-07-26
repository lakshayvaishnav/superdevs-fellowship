1. declarative macros
   ✅ Features:
   Pattern-matching macro syntax

Built into the Rust compiler

Easier and safer to use than procedural macros

Used for small syntactic sugar, DSLs, and generic code

```
macro_rules! name {
(pattern) => { expansion };
}

```

2. procedural macros

Procedural macros give you more control by letting you manipulate Rust syntax as token streams. They require an external crate with the attribute:
proc-macro = true

    🔸 2.1. Derive Macros

        📌 Used with: #[derive(MyTrait)]
        Automatically implements traits for structs or enums.

    ```
    #[derive(Debug, Serialize, Deserialize)]
    struct User {
    name: String,
    }
    ```

    🔸 2.2. Attribute-like Macros

        📌 Used with: #[my_attribute]
        Adds behavior to functions, structs, enums, etc.

    ```
        #[route(GET, "/home")]
        fn home() { }
    ```

    🔸 2.3. Function-like Macros
    
        📌 Used like: my_macro!(...)
        Look similar to macro_rules!, but written in Rust code using token stream logic.

    ```
    my_macro!(name, type);

    #[proc_macro]
    pub fn my_macro(input: TokenStream) -> TokenStream {
        // Parse input tokens and return generated tokens
    }
    ```

procedural macros have to be defined in their own crate...