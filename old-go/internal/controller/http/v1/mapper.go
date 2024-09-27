package v1

import (
	"justshake/cocktails/internal/use_cases/cocktails"
)

func mapToCocktailResponseItem(items []cocktails.CocktailResponseItem) []cocktailResponseItem {
	result := make([]cocktailResponseItem, len(items))
	for i, item := range items {
		result[i] = cocktailResponseItem{
			Id:          item.Id,
			Name:        item.Name,
			RussianName: item.RussianName,
			History:     item.History,
			Tags:        mapToTagApiResponse(item.Tags),
		}
	}
	return result
}

func mapToTagApiResponse(items []cocktails.TagResponseItem) []tagApiResponse {
	result := make([]tagApiResponse, len(items))
	for i, item := range items {
		result[i] = tagApiResponse{Name: item.Name}
	}
	return result
}

func mapToCocktailItemApiResponse(items []cocktails.CocktailItemResponseItem) []cocktailItemApiResponse {
	result := make([]cocktailItemApiResponse, len(items))
	for i, item := range items {
		result[i] = cocktailItemApiResponse{
			Name:  item.Name,
			Count: item.Count,
			Unit:  item.Unit,
		}
	}
	return result
}

func mapToRecipeApiResponse(item cocktails.RecipeResponseItem) recipeApiResponse {
	return recipeApiResponse{Steps: item.Steps}
}
