package use_cases

import (
	"context"
	"fmt"
	cocktail_aggregate "justshake/cocktails/internal/domain/cocktail-aggregate"
	"justshake/cocktails/internal/infrastructure/repositories"
)

type CocktailsUseCase struct {
	cocktailsRepo repositories.CocktailsRepo
}

func New(cocktailsRepo repositories.CocktailsRepo) *CocktailsUseCase {
	return &CocktailsUseCase{cocktailsRepo}
}

func (uc *CocktailsUseCase) GetById(ctx context.Context, request GetByIdRequest) (GetByIdResponse, error) {
	result, err := uc.cocktailsRepo.GetById(request.Id)
	if err != nil {
		return GetByIdResponse{}, fmt.Errorf("CocktailsUseCase - GetById - uc.cocktailsRepo.GetById: %w", err)
	}
	return GetByIdResponse{
		Id:                  result.Id,
		Name:                result.Name,
		RussianName:         result.RussianName,
		History:             result.History,
		Tags:                mapToTagResponseItem(result.Tags),
		Tools:               mapToCocktailsItemResponseItem(result.Tools),
		CompositionElements: mapToCocktailsItemResponseItem(result.CompositionElements),
		Recipe:              RecipeResponseItem{result.Recipe.Steps},
	}, nil
}

func (uc *CocktailsUseCase) GetByFilter(ctx context.Context, req GetByFilterRequest) (GetByFilterResponse, error) {
	result, err := uc.cocktailsRepo.GetByFilter(cocktail_aggregate.CocktailFilter{
		Ids:        req.Ids,
		Pagination: req.Pagination,
	})

	if err != nil {
		return GetByFilterResponse{}, fmt.Errorf("CocktailsUseCase - GetByFilter - uc.cocktailsRepo.GetByFilter: %w", err)
	}

	return GetByFilterResponse{
		Items:      mapToCocktailResponseItem(result.Items),
		TotalItems: result.TotalCount,
	}, nil
}
