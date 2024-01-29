package users

import "github.com/google/uuid"

type (
	// CreateUserRequest Запрос на создание нового пользователя
	CreateUserRequest struct {
		Id         uuid.UUID
		TelegramId int64
	}
	// DeleteUserRequest Запрос на удаление пользователя
	DeleteUserRequest struct {
		TelegramId int64
	}
	// GetByTelegramIdRequest Запрос на получение пользователя по идентификатору Telegram
	GetByTelegramIdRequest struct {
		Id int64
	}
	// UserResponse Данные о пользователе
	UserResponse struct {
		Id                uuid.UUID
		TelegramId        int64
		FavoriteCocktails []uuid.UUID
	}

	FavoriteCocktailRequest struct {
		TelegramId int64
		CocktailId uuid.UUID
	}
)
