#![allow(dead_code)]
// Each handler is supposed to have a comment above it describing its purpose.

// handler to used to fetch example data
pub async fn example_list() -> &'static str {
    "List all examples"
}

// handler to create a new example
pub async fn example_create() -> &'static str {
    "Create an example"
}

// handler to fetch a specific example by ID
pub async fn example_detail() -> &'static str {
    "Get example detail"
}

// handler to update an existing example by ID
pub async fn example_update() -> &'static str {
    "Update example"
}

// handler to delete an example by ID
pub async fn example_delete() -> &'static str {
    "Delete example"
}