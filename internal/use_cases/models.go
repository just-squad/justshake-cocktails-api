package use_cases

import (
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
		CountryOfOrigin     string
		History             string
		Tags                []TagResponseItem
		Tools               []CocktailItemResponseItem
		CompositionElements []CocktailItemResponseItem
		Recipe              RecipeResponseItem
	}
	GetByFilterRequest struct {
		Ids          []uuid.UUID
		Names        []string
		RussianNames []string
		Pagination   models.Pagination
	}
	GetByFilterResponse struct {
		Items      []CocktailResponseItem
		TotalItems int64
	}
	CocktailResponseItem struct {
		Id              uuid.UUID
		Name            string
		RussianName     string
		CountryOfOrigin string
		History         string
		Tags            []TagResponseItem
	}
	TagResponseItem struct {
		Name string
	}
	CocktailItemResponseItem struct {
		Name  string
		Count int
		Unit  string
	}
	RecipeResponseItem struct {
		Steps []string
	}
	GetNamesRequest struct {
		Pagination models.Pagination
	}
	GetNamesResponse struct {
		Items      []GetNamesItemResponse
		TotalItems int64
	}
	GetNamesItemResponse struct {
		Id   uuid.UUID
		Name string
	}
)
