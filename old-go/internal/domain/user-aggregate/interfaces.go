package user_aggregate

import (
	"context"
)

type (
	UsersRepo interface {
		Create(ctx context.Context, entity User) error
		Delete(ctx context.Context, entity User) error
		Update(ctx context.Context, entity User) error
		GetByTelegramId(ctx context.Context, telegramId int64) (User, error)
		IsExist(ctx context.Context, telegramId int64) (bool, error)
	}
)
