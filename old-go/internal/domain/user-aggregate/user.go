package user_aggregate

import "github.com/google/uuid"

type User struct {
	Id                uuid.UUID   `bson:"id" json:"id"`
	TelegramId        int64       `bson:"telegram_id" json:"telegram_id"`
	FavoriteCocktails []uuid.UUID `bson:"favorite_cocktails" json:"favorite_cocktails"`
}
