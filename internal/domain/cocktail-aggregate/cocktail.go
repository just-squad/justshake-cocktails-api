package cocktail_aggregate

import (
	"github.com/google/uuid"
	"justshake/cocktails/internal/domain/models"
)

type CocktailFilter struct {
	Ids          []uuid.UUID
	Names        []string
	RussianNames []string
	Pagination   models.Pagination
}

type (
	CocktailsPaged struct {
		Items      []Cocktail
		TotalCount int64
	}
)

type Cocktail struct {
	Id                  uuid.UUID      `bson:"id" json:"id"`
	Url                 string         `bson:"url" json:"url"`
	Name                string         `bson:"name" json:"name"`
	RussianName         string         `bson:"russian_name" json:"russian_name"`
	CountryOfOrigin     string         `bson:"country_of_origin" json:"country_of_origin"`
	History             string         `bson:"history" json:"history"`
	Tags                []Tag          `bson:"tags" json:"tags"`
	Tools               []CocktailItem `bson:"tools" json:"tools"`
	CompositionElements []CocktailItem `bson:"composition_elements" json:"composition_elements"`
	Recipe              Recipe         `bson:"recipe" json:"recipe"`
}

func (cock *Cocktail) SetId() {
	cock.Id = uuid.New()
}

type Tag struct {
	Name string `bson:"name" json:"name"`
}

type CocktailItem struct {
	Name  string `bson:"name" json:"name"`
	Count int    `bson:"count" json:"count"`
	Unit  string `bson:"unit" json:"unit"`
}

type Recipe struct {
	Steps []string `bson:"steps" json:"steps"`
}
