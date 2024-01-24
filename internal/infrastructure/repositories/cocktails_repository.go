package repositories

import (
	"context"
	"fmt"
	"github.com/google/uuid"
	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
	"justshake/cocktails/internal/domain/cocktail-aggregate"
	"justshake/cocktails/internal/domain/models"
	"justshake/cocktails/pkg/logger"
	"justshake/cocktails/pkg/mng"
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

func (cr *CocktailsRepository) Create(ctx context.Context, entity cocktail_aggregate.Cocktail) error {
	collection := cr.getCollection()
	cr.l.Info(entity.Id.String())
	entity.SetId()

	// Insert a single document
	insertResult, err := collection.InsertOne(ctx, entity)
	if err != nil {
		cr.l.Error(err)
		return err
	}
	fmt.Println("Inserted a single document: ", insertResult.InsertedID)
	return nil
}

func (cr *CocktailsRepository) GetNames(ctx context.Context, pagination models.Pagination) (cocktail_aggregate.CocktailsPaged, error) {
	collection := cr.getCollection()

	findOptions := options.Find()
	if pagination.Page == 0 && pagination.ItemsPerPage == 0 {
		findOptions.SetLimit(10)
	} else {
		findOptions.SetSkip(pagination.Page * pagination.ItemsPerPage)
		findOptions.SetLimit(pagination.ItemsPerPage)
	}
	queryFilter := bson.M{}
	// Finding multiple documents returns a cursor
	cur, err := collection.Find(ctx, queryFilter, findOptions)
	if err != nil {
		cr.l.Error(err)
		return cocktail_aggregate.CocktailsPaged{}, err
	}

	var results []cocktail_aggregate.Cocktail
	// Iterate through the cursor
	for cur.Next(ctx) {
		elem := struct {
			Id          uuid.UUID `bson:"id" json:"id"`
			RussianName string    `bson:"russian_name" json:"russian_name"`
		}{}
		err := cur.Decode(&elem)
		if err != nil {
			cr.l.Error(err)
		}

		results = append(results, cocktail_aggregate.Cocktail{Id: elem.Id, RussianName: elem.RussianName})
	}
	if err := cur.Err(); err != nil {
		cr.l.Error(err)
		return cocktail_aggregate.CocktailsPaged{}, err
	}

	// Close the cursor once finished
	err = cur.Close(ctx)
	if err != nil {
		cr.l.Error(err)
		return cocktail_aggregate.CocktailsPaged{}, err
	}

	count, err := collection.CountDocuments(ctx, queryFilter)
	if err != nil {
		cr.l.Error(err)
		return cocktail_aggregate.CocktailsPaged{}, err
	}

	return cocktail_aggregate.CocktailsPaged{
		Items:      results,
		TotalCount: count,
	}, nil
}

func (cr *CocktailsRepository) GetById(ctx context.Context, id uuid.UUID) (cocktail_aggregate.Cocktail, error) {
	collection := cr.getCollection()

	var result cocktail_aggregate.Cocktail
	filter := bson.D{{"id", id}}

	err := collection.FindOne(ctx, filter).Decode(&result)
	if err != nil {
		cr.l.Error(err)
		return cocktail_aggregate.Cocktail{}, err
	}

	fmt.Printf("Found a single document: %+v\n", result)

	return result, nil
}

func (cr *CocktailsRepository) GetByFilter(ctx context.Context, filter cocktail_aggregate.CocktailFilter) (cocktail_aggregate.CocktailsPaged, error) {
	collection := cr.getCollection()

	findOptions := options.Find()
	if filter.Pagination.Page == 0 && filter.Pagination.ItemsPerPage == 0 {
		findOptions.SetLimit(10)
	} else {
		findOptions.SetSkip(filter.Pagination.Page * filter.Pagination.ItemsPerPage)
		findOptions.SetLimit(filter.Pagination.ItemsPerPage)
	}
	queryFilterIds := bson.M{}
	if len(filter.Ids) > 0 {
		f := bson.A{}
		for _, id := range filter.Ids {
			f = append(f, bson.M{"id": id})
		}
		queryFilterIds["$or"] = f
	}
	queryFilterNames := bson.M{}
	if len(filter.Names) > 0 {
		f := bson.A{}
		for _, name := range filter.Names {
			f = append(f, bson.D{{"name", name}})
		}
		queryFilterNames["$or"] = f
	}
	queryFilterRussianNames := bson.M{}
	if len(filter.RussianNames) > 0 {
		f := bson.A{}
		for _, name := range filter.RussianNames {
			f = append(f, bson.D{{"russian_name", name}})
		}
		queryFilterNames["$or"] = f
	}
	queryFilter := bson.M{}
	f := bson.A{}
	if len(queryFilterIds) > 0 {
		f = append(f, queryFilterIds)
	}
	if len(queryFilterNames) > 0 {
		f = append(f, queryFilterNames)
	}
	if len(queryFilterRussianNames) > 0 {
		f = append(f, queryFilterRussianNames)
	}
	if len(f) > 0 {
		queryFilter["$and"] = f
	}

	var results []cocktail_aggregate.Cocktail

	// Finding multiple documents returns a cursor
	cur, err := collection.Find(ctx, queryFilter, findOptions)
	if err != nil {
		cr.l.Error(err)
		return cocktail_aggregate.CocktailsPaged{}, err
	}

	// Iterate through the cursor
	for cur.Next(ctx) {
		var elem cocktail_aggregate.Cocktail
		err := cur.Decode(&elem)
		if err != nil {
			cr.l.Error(err)
		}

		results = append(results, elem)
	}

	if err := cur.Err(); err != nil {
		cr.l.Error(err)
		return cocktail_aggregate.CocktailsPaged{}, err
	}

	// Close the cursor once finished
	err = cur.Close(ctx)
	if err != nil {
		cr.l.Error(err)
		return cocktail_aggregate.CocktailsPaged{}, err
	}

	count, err := collection.CountDocuments(ctx, queryFilter)
	if err != nil {
		cr.l.Error(err)
		return cocktail_aggregate.CocktailsPaged{}, err
	}

	fmt.Printf("Found multiple documents : %+v elements\n", len(results))

	return cocktail_aggregate.CocktailsPaged{
		Items:      results,
		TotalCount: count,
	}, nil
}
