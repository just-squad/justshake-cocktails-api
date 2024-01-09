package repositories

import (
	"context"
	"fmt"
	"github.com/google/uuid"
	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
	"justshake/cocktails/internal/domain/cocktail-aggregate"
	"justshake/cocktails/pkg/logger"
	"justshake/cocktails/pkg/mng"
)

type (
	CocktailsPaged struct {
		Items      []cocktail_aggregate.Cocktail
		TotalCount int64
	}
)

type CocktailsRepository struct {
	mongo *mng.Connection
	l     logger.Interface
}

func New(mongo *mng.Connection, l logger.Interface) *CocktailsRepository {

	return &CocktailsRepository{mongo: mongo, l: l}
}

func (cr *CocktailsRepository) getCollection() *mongo.Collection {
	return cr.mongo.Client.Database(cr.mongo.Database).Collection("cocktails")
}

func (cr *CocktailsRepository) Create(entity cocktail_aggregate.Cocktail) error {
	collection := cr.getCollection()
	cr.l.Info(entity.Id.String())
	entity.SetId()

	// Insert a single document
	insertResult, err := collection.InsertOne(context.TODO(), entity)
	if err != nil {
		cr.l.Error(err)
		return err
	}
	fmt.Println("Inserted a single document: ", insertResult.InsertedID)
	return nil
}

func (cr *CocktailsRepository) GetById(id uuid.UUID) (cocktail_aggregate.Cocktail, error) {
	collection := cr.getCollection()

	var result cocktail_aggregate.Cocktail
	filter := bson.D{{"id", id}}

	err := collection.FindOne(context.TODO(), filter).Decode(&result)
	if err != nil {
		cr.l.Error(err)
		return cocktail_aggregate.Cocktail{}, err
	}

	fmt.Printf("Found a single document: %+v\n", result)

	return result, nil
}

func (cr *CocktailsRepository) GetByFilter(filter cocktail_aggregate.CocktailFilter) (CocktailsPaged, error) {
	collection := cr.getCollection()

	findOptions := options.Find()
	if filter.Pagination.Page == 0 && filter.Pagination.ItemsPerPage == 0 {
		findOptions.SetLimit(10)
	} else {
		findOptions.SetSkip(filter.Pagination.Page * filter.Pagination.ItemsPerPage)
		findOptions.SetLimit(filter.Pagination.ItemsPerPage)
	}
	queryFilter := bson.M{}
	if len(filter.Ids) > 0 {
		f := bson.A{}
		for _, id := range filter.Ids {
			f = append(f, bson.M{"id": id})
		}
		queryFilter["$or"] = f
	}

	var results []cocktail_aggregate.Cocktail

	// Finding multiple documents returns a cursor
	cur, err := collection.Find(context.TODO(), queryFilter, findOptions)
	if err != nil {
		cr.l.Error(err)
		return CocktailsPaged{}, err
	}

	// Iterate through the cursor
	for cur.Next(context.TODO()) {
		var elem cocktail_aggregate.Cocktail
		err := cur.Decode(&elem)
		if err != nil {
			cr.l.Error(err)
		}

		results = append(results, elem)
	}

	if err := cur.Err(); err != nil {
		cr.l.Error(err)
		return CocktailsPaged{}, err
	}

	// Close the cursor once finished
	err = cur.Close(context.TODO())
	if err != nil {
		cr.l.Error(err)
		return CocktailsPaged{}, err
	}

	count, err := collection.CountDocuments(context.TODO(), queryFilter)
	if err != nil {
		cr.l.Error(err)
		return CocktailsPaged{}, err
	}

	fmt.Printf("Found multiple documents : %+v elements\n", len(results))

	return CocktailsPaged{
		Items:      results,
		TotalCount: count,
	}, nil
}
