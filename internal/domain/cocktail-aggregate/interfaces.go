package cocktail_aggregate

import (
	"context"
	"github.com/google/uuid"
	"justshake/cocktails/internal/domain/models"
)

type (
	CocktailsRepo interface {
		Create(ctx context.Context, entity Cocktail) error
		GetNames(ctx context.Context, pagination models.Pagination) (CocktailsPaged, error)
		GetById(ctx context.Context, id uuid.UUID) (Cocktail, error)
		GetByFilter(ctx context.Context, filter CocktailFilter) (CocktailsPaged, error)
	}
)
