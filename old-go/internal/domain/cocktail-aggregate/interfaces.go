package cocktail_aggregate

import (
	"context"
	"github.com/google/uuid"
)

type (
	CocktailsRepo interface {
		Create(ctx context.Context, entity Cocktail) error
		GetNames(ctx context.Context, filter CocktailNamesFilter) (CocktailsPaged, error)
		GetById(ctx context.Context, id uuid.UUID) (Cocktail, error)
		GetByFilter(ctx context.Context, filter CocktailFilter) (CocktailsPaged, error)
	}
)
