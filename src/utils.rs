

pub fn authenticate(username: &str, password: &str) -> bool {
    if username == "sangit" && password == "sangit@123" {
        true
    } else {
        false
    }
}