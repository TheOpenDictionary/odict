pub trait SnakeCase {
    fn snake_case(&self) -> String;
}

impl SnakeCase for str {
    fn snake_case(&self) -> String {
        let mut snake = String::new();

        for (i, ch) in self.char_indices() {
            if i > 0 && ch.is_uppercase() {
                snake.push('_');
            }
            snake.push(ch.to_ascii_lowercase());
        }

        snake
    }
}
