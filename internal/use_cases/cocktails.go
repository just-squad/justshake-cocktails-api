package use_cases

import (
	"context"
	"fmt"
	cocktail_aggregate "justshake/cocktails/internal/domain/cocktail-aggregate"
	"justshake/cocktails/internal/domain/models"
	"justshake/cocktails/internal/use_cases/cocktails"
)

type CocktailsUseCase struct {
	cocktailsRepo cocktail_aggregate.CocktailsRepo
}

func New(cocktailsRepo cocktail_aggregate.CocktailsRepo) *CocktailsUseCase {
	return &CocktailsUseCase{cocktailsRepo}
}

func (uc *CocktailsUseCase) GetNames(ctx context.Context, req cocktails.GetNamesRequest) (cocktails.GetNamesResponse, error) {
	res, err := uc.cocktailsRepo.GetNames(ctx, models.Pagination{
		Page:         req.Pagination.Page,
		ItemsPerPage: req.Pagination.ItemsPerPage,
	})
	if err != nil {
		return cocktails.GetNamesResponse{}, fmt.Errorf("CocktailsUseCase - Getnames - uc.cocktailsRepo.GetNames: %w", err)
	}

	it := make([]cocktails.GetNamesItemResponse, len(res.Items))
	for i, item := range res.Items {
		it[i] = cocktails.GetNamesItemResponse{
			Id:   item.Id,
			Name: item.RussianName,
		}
	}
	return cocktails.GetNamesResponse{
		Items:      it,
		TotalItems: res.TotalCount,
	}, nil
}

func (uc *CocktailsUseCase) GetById(ctx context.Context, request cocktails.GetByIdRequest) (cocktails.GetByIdResponse, error) {
	result, err := uc.cocktailsRepo.GetById(ctx, request.Id)
	if err != nil {
		return cocktails.GetByIdResponse{}, fmt.Errorf("CocktailsUseCase - GetById - uc.cocktailsRepo.GetById: %w", err)
	}
	return cocktails.GetByIdResponse{
		Id:                  result.Id,
		Name:                result.Name,
		RussianName:         result.RussianName,
		CountryOfOrigin:     result.CountryOfOrigin,
		History:             result.History,
		Tags:                mapToTagResponseItem(result.Tags),
		Tools:               mapToCocktailsItemResponseItem(result.Tools),
		CompositionElements: mapToCocktailsItemResponseItem(result.CompositionElements),
		Recipe:              cocktails.RecipeResponseItem{result.Recipe.Steps},
	}, nil
}

func (uc *CocktailsUseCase) GetByFilter(ctx context.Context, req cocktails.GetByFilterRequest) (cocktails.GetByFilterResponse, error) {
	result, err := uc.cocktailsRepo.GetByFilter(ctx, cocktail_aggregate.CocktailFilter{
		Ids:          req.Ids,
		Names:        req.Names,
		RussianNames: req.RussianNames,
		Pagination:   req.Pagination,
	})

	if err != nil {
		return cocktails.GetByFilterResponse{}, fmt.Errorf("CocktailsUseCase - GetByFilter - uc.cocktailsRepo.GetByFilter: %w", err)
	}

	return cocktails.GetByFilterResponse{
		Items:      mapToCocktailResponseItem(result.Items),
		TotalItems: result.TotalCount,
	}, nil
}
