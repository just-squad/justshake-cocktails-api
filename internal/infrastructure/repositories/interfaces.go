package repositories

import (
	"github.com/google/uuid"
	cocktailAggregate "justshake/cocktails/internal/domain/cocktail-aggregate"
)

type (
	CocktailsRepo interface {
		Create(entity cocktailAggregate.Cocktail) error
		GetById(id uuid.UUID) (cocktailAggregate.Cocktail, error)
		GetByFilter(filter cocktailAggregate.CocktailFilter) (CocktailsPaged, error)
	}
)
