package cocktails_repository

import (
	"github.com/google/uuid"
	"justshake/cocktails/internal/domain/cocktail-aggregate"
)

type (
	CocktailsPaged struct {
		Items      cocktail_aggregate.Cocktail
		Page       int
		TotalCount int
	}
)

func New() *CocktailsRepository {
	return &CocktailsRepository{}
}

type CocktailsRepository struct {
}

func (cr *CocktailsRepository) GetById(id uuid.UUID) (cocktail_aggregate.Cocktail, error) {
	return cocktail_aggregate.Cocktail{
		uuid.New(),
		"",
		"",
		"",
		nil,
		nil,
		nil,
		cocktail_aggregate.Recipe{},
	}, nil
}

func (cr *CocktailsRepository) GetByFilter(filter cocktail_aggregate.CocktailFilter) (CocktailsPaged, error) {
	return CocktailsPaged{
		Items:      cocktail_aggregate.Cocktail{},
		Page:       0,
		TotalCount: 0,
	}, nil
}
