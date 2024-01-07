package use_cases

import (
	"context"
	"github.com/google/uuid"
	"justshake/cocktails/internal/domain/models"
)

// Модели
type (
	GetByIdRequest struct {
		Id uuid.UUID
	}
	GetByIdResponse struct {
		Id                  uuid.UUID
		Name                string
		RussianName         string
		History             string
		Tags                []TagResponseItem
		Tools               []CocktailItemResponseItem
		CompositionElements []CocktailItemResponseItem
		Recipe              RecipeResponseItem
	}
	GetByFilterRequest struct {
		Ids        []uuid.UUID
		Pagination models.Pagination
	}
	GetByFilterResponse struct {
		Items      []CocktailResponseItem
		Page       int
		TotalItems int
	}
	CocktailResponseItem struct {
		Id          uuid.UUID
		Name        string
		RussianName string
		History     string
		Tags        []TagResponseItem
	}
	TagResponseItem struct {
		Name string
	}
	CocktailItemResponseItem struct {
		Id    int64
		Name  string
		Count int32
		Unit  string
	}
	RecipeResponseItem struct {
		Steps []string
	}
)

// Cocktails Интерфейсы
type (
	Cocktails interface {
		GetById(ctx context.Context, req GetByIdRequest) (GetByIdResponse, error)
		GetByFilter(ctx context.Context, req GetByFilterRequest) (GetByFilterResponse, error)
	}
)
