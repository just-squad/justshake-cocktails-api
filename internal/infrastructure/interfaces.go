package cocktails_repository

import (
	"github.com/google/uuid"
	cocktailAggregate "justshake/cocktails/internal/domain/cocktail-aggregate"
)

type (
	CocktailsRepo interface {
		GetById(i1d uuid.UUID) (cocktailAggregate.Cocktail, error)
		GetByFilter(filter cocktailAggregate.CocktailFilter) (CocktailsPaged, error)
	}
)
