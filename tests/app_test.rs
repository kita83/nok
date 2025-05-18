use nok::app::{App, User, UserStatus};

#[test]
fn test_knock_feature() {
    let mut app = App::new();
    app.users.push(User::new("TestUser".to_string()));
    
    app.knock("TestUser");
    
    assert!(app.notification.is_some());
    let notification = app.notification.unwrap();
    assert!(notification.contains("KON KON"));
    assert!(notification.contains("TestUser"));
}
