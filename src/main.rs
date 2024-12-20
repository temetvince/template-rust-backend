mod crud_service;
mod db;
mod models;

#[tokio::main]
async fn main() {
    // Example usage of the `CrudService`.
    let crud_service = crud_service::CrudService::new().await;

    // Create a new user.
    let user = models::User {
        id: None,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    };

    crud_service
        .create_user(&user)
        .await
        .expect("Failed to create user");

    // Retrieve a user by ID.
    if let Some(user) = crud_service.get_user(1).await.expect("Failed to get user") {
        println!("User found: {:?}", user);
    } else {
        println!("User not found");
    }

    // List all users.
    let users = crud_service.list_users().await.expect("Failed to list users");
    println!("All users: {:?}", users);

    // Update a user's information.
    let updated_user = models::User {
        id: Some(1),
        name: "Alice Smith".to_string(),
        email: "alice.smith@example.com".to_string(),
    };

    crud_service
        .update_user(&updated_user)
        .await
        .expect("Failed to update user");

    // Delete a user.
    crud_service
        .delete_user(1)
        .await
        .expect("Failed to delete user");
}
