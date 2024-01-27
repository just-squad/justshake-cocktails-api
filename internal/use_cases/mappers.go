package use_cases

import (
	cocktail_aggregate "justshake/cocktails/internal/domain/cocktail-aggregate"
	"justshake/cocktails/internal/use_cases/cocktails"
)

func mapToTagResponseItem(items []cocktail_aggregate.Tag) []cocktails.TagResponseItem {
	result := make([]cocktails.TagResponseItem, len(items))
	for i, item := range items {
		result[i] = cocktails.TagResponseItem{
			Name: item.Name,
		}
	}
	return result
}

func mapToCocktailsItemResponseItem(items []cocktail_aggregate.CocktailItem) []cocktails.CocktailItemResponseItem {
	result := make([]cocktails.CocktailItemResponseItem, len(items))
	for i, item := range items {
		result[i] = cocktails.CocktailItemResponseItem{
			Name:  item.Name,
			Count: item.Count,
			Unit:  item.Unit,
		}
	}
	return result
}

func mapToCocktailResponseItem(items []cocktail_aggregate.Cocktail) []cocktails.CocktailResponseItem {
	result := make([]cocktails.CocktailResponseItem, len(items))
	for i, item := range items {
		result[i] = cocktails.CocktailResponseItem{
			Id:              item.Id,
			Name:            item.Name,
			RussianName:     item.RussianName,
			CountryOfOrigin: item.CountryOfOrigin,
			History:         item.History,
			Tags:            mapToTagResponseItem(item.Tags),
		}
	}
	return result
}
