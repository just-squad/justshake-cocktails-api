#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    ReceiveCocktailName,
    ReveivedCocktailName {
        cocktail_name: String,
    },
}
