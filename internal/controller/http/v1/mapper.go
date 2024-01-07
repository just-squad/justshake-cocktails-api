package v1

import (
	"justshake/cocktails/internal/use_cases"
)

func mapToCocktailResponseItem(items []use_cases.CocktailResponseItem) []cocktailResponseItem {
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

func mapToTagApiResponse(items []use_cases.TagResponseItem) []tagApiResponse {
	result := make([]tagApiResponse, len(items))
	for i, item := range items {
		result[i] = tagApiResponse{Name: item.Name}
	}
	return result
}

func mapToCocktailItemApiResponse(items []use_cases.CocktailItemResponseItem) []cocktailItemApiResponse {
	result := make([]cocktailItemApiResponse, len(items))
	for i, item := range items {
		result[i] = cocktailItemApiResponse{
			Id:    item.Id,
			Name:  item.Name,
			Count: item.Count,
			Unit:  item.Unit,
		}
	}
	return result
}

func mapToRecipeApiResponse(item use_cases.RecipeResponseItem) recipeApiResponse {
	return recipeApiResponse{Steps: item.Steps}
}
