use crate::service_provider::ServiceProviderInterface;
use crate::tests::mock::mock_default_app_state;

#[test]
fn test_find() {
    let app = mock_default_app_state();

    let user = app
        .bot_user_service()
        .create_and_save_bot_user(1337, "test".to_string())
        .unwrap();

    let found_user = app.bot_user_repository().find(user.id).unwrap().unwrap();
    assert_eq!(user, found_user);
}

#[test]
fn test_save() {
    let app = mock_default_app_state();

    let mut user = app
        .bot_user_service()
        .create_and_save_bot_user(1337, "test".to_string())
        .unwrap();
    user.username = "cookie".to_string();

    app.bot_user_repository().save(user.clone()).unwrap();
    let found_user = app.bot_user_repository().find(1337).unwrap().unwrap();
    assert_eq!(found_user.username, "cookie".to_string());
}

#[test]
fn test_delete() {
    let app = mock_default_app_state();

    let user = app
        .bot_user_service()
        .create_and_save_bot_user(1337, "test".to_string())
        .unwrap();

    let found_user = app.bot_user_repository().find(user.id).unwrap().unwrap();
    assert_eq!(user, found_user);

    let found_user_id = found_user.id;
    app.bot_user_repository().delete(found_user).unwrap();
    assert_eq!(app.bot_user_repository().find(found_user_id).unwrap(), None);
}
