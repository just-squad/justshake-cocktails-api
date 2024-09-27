package users

import "context"

// Users Интерфейс для структуры обработки бизнес логики
type (
	Users interface {
		Create(ctx context.Context, req CreateUserRequest) error
		Delete(ctx context.Context, req DeleteUserRequest) error
		IsExist(ctx context.Context, req GetByTelegramIdRequest) (bool, error)
		GetByTelegramId(ctx context.Context, req GetByTelegramIdRequest) (UserResponse, error)
		AddCocktailToFavorite(ctx context.Context, req FavoriteCocktailRequest) error
		RemoveCocktailFromFavorite(ctx context.Context, req FavoriteCocktailRequest) error
	}
)
