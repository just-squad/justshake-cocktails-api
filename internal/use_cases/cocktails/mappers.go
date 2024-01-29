package cocktails

import (
	cocktail_aggregate "justshake/cocktails/internal/domain/cocktail-aggregate"
)

func mapToTagResponseItem(items []cocktail_aggregate.Tag) []TagResponseItem {
	result := make([]TagResponseItem, len(items))
	for i, item := range items {
		result[i] = TagResponseItem{
			Name: item.Name,
		}
	}
	return result
}

func mapToCocktailsItemResponseItem(items []cocktail_aggregate.CocktailItem) []CocktailItemResponseItem {
	result := make([]CocktailItemResponseItem, len(items))
	for i, item := range items {
		result[i] = CocktailItemResponseItem{
			Name:  item.Name,
			Count: item.Count,
			Unit:  item.Unit,
		}
	}
	return result
}

func mapToCocktailResponseItem(items []cocktail_aggregate.Cocktail) []CocktailResponseItem {
	result := make([]CocktailResponseItem, len(items))
	for i, item := range items {
		result[i] = CocktailResponseItem{
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
