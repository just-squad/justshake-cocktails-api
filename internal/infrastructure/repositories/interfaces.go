package repositories

import (
	"context"
	"github.com/google/uuid"
	cocktailAggregate "justshake/cocktails/internal/domain/cocktail-aggregate"
)

type (
	CocktailsRepo interface {
		Create(ctx context.Context, entity cocktailAggregate.Cocktail) error
		GetById(ctx context.Context, id uuid.UUID) (cocktailAggregate.Cocktail, error)
		GetByFilter(ctx context.Context, filter cocktailAggregate.CocktailFilter) (CocktailsPaged, error)
	}
)
