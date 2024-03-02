use sqlx::PgPool;
use crate::core::contracts::user::RegisterUserData;
use crate::core::persistence::persistence;
use crate::core::utils::test_helper;

// #[sqlx::test(migrator = "persistence::MIGRATOR")]
// async fn register_user_test(pool: PgPool) -> sqlx::Result<()> {
//
//     let context = test_helper::create_execution_context(pool, None).await;
//     let register_data = RegisterUserData {
//         name: "testname".to_string(),
//         email: "tes@email.com".to_string(),
//         password: "password".to_string(),
//     };
//     context.auth_provider.register_user(&context, register_data)
//
// }