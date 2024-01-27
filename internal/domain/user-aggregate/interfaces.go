package user_aggregate

import (
	"context"
)

type (
	UsersRepo interface {
		Create(ctx context.Context, entity User) error
		Update(ctx context.Context, entity User) error
		GetByTelegramId(ctx context.Context, telegramId string) (User, error)
	}
)
