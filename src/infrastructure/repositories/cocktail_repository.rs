use crate::domain::aggregates::cocktail::CocktailRepo;

#[derive(Debug, Clone)]
struct CocktailRepository {
}

impl CocktailRepo for CocktailRepository {
    async fn create(&self) {
        todo!()
    }

    async fn get_names(&self) {
        todo!()
    }

    async fn get_by_id(&self) {
        todo!()
    }

    async fn get_by_filter(&self) {
        todo!()
    }
}
