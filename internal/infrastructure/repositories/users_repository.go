package repositories

import (
	"context"
	"fmt"
	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/mongo"
	user_aggregate "justshake/cocktails/internal/domain/user-aggregate"
	"justshake/cocktails/pkg/logger"
	"justshake/cocktails/pkg/mng"
)

type UsersRepository struct {
	mongo *mng.Connection
	l     logger.Interface
}

func NewUsersRepository(mongo *mng.Connection, l logger.Interface) *UsersRepository {
	return &UsersRepository{mongo: mongo, l: l}
}

func (ur *UsersRepository) getCollection() *mongo.Collection {
	return ur.mongo.Client.Database(ur.mongo.Database).Collection("users")
}

func (ur *UsersRepository) Create(ctx context.Context, entity user_aggregate.User) error {
	collection := ur.getCollection()
	ur.l.Info(entity.Id.String())

	// Insert a single document
	insertResult, err := collection.InsertOne(ctx, entity)
	if err != nil {
		ur.l.Error(err)
		return err
	}
	fmt.Println("Inserted a single document: ", insertResult.InsertedID)
	return nil
}

func (ur *UsersRepository) Update(ctx context.Context, entity user_aggregate.User) error {
	collection := ur.getCollection()
	ur.l.Info(entity.Id.String())

	// Insert a single document
	insertResult, err := collection.UpdateOne(ctx, bson.D{{Key: "telegram_id", Value: entity.TelegramId}}, entity)
	if err != nil {
		ur.l.Error(err)
		return err
	}
	fmt.Println("Updated a single document: ", insertResult.UpsertedID)
	return nil
}

func (ur *UsersRepository) GetByTelegramId(ctx context.Context, telegramId string) (user_aggregate.User, error) {
	collection := ur.getCollection()

	var result user_aggregate.User
	filter := bson.D{{Key: "telegram_id", Value: telegramId}}

	err := collection.FindOne(ctx, filter).Decode(&result)
	if err != nil {
		ur.l.Error(err)
		return user_aggregate.User{}, err
	}

	fmt.Printf("Found a single document: %+v\n", result)

	return result, nil
}
