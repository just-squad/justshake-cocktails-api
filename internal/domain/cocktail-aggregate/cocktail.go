package cocktail_aggregate

import (
	"github.com/google/uuid"
	"justshake/cocktails/internal/domain/models"
)

type CocktailFilter struct {
	Ids        []uuid.UUID
	Pagination models.Pagination
}

type Cocktail struct {
	Id                  uuid.UUID
	Name                string
	RussianName         string
	History             string
	Tags                []Tag
	Tools               []CocktailItem
	CompositionElements []CocktailItem
	Recipe              Recipe
}

type Tag struct {
	Name string
}

type CocktailItem struct {
	Id    int64
	Name  string
	Count int32
	Unit  string
}

type Recipe struct {
	Steps []string
}
