package users

import (
	"context"
	"fmt"
	user_aggregate "justshake/cocktails/internal/domain/user-aggregate"
	"justshake/cocktails/internal/use_cases"
	"justshake/cocktails/internal/use_cases/cocktails"
)

type UsersUseCase struct {
	usersRepo user_aggregate.UsersRepo
}

func NewUsersCases(cocktailsRepo user_aggregate.UsersRepo) *UsersUseCase {
	return &UsersUseCase{cocktailsRepo}
}

func (uc *UsersUseCase) GetByTelegramId(ctx context.Context, request cocktails.GetByIdRequest) (cocktails.GetByIdResponse, error) {
	result, err := uc.usersRepo.GetByTelegramId(ctx, request.Id)
	if err != nil {
		return cocktails.GetByIdResponse{}, fmt.Errorf("CocktailsUseCase - GetById - uc.cocktailsRepo.GetById: %w", err)
	}
	return cocktails.GetByIdResponse{
		Id:                  result.Id,
		Name:                result.Name,
		RussianName:         result.RussianName,
		CountryOfOrigin:     result.CountryOfOrigin,
		History:             result.History,
		Tags:                use_cases.mapToTagResponseItem(result.Tags),
		Tools:               use_cases.mapToCocktailsItemResponseItem(result.Tools),
		CompositionElements: use_cases.mapToCocktailsItemResponseItem(result.CompositionElements),
		Recipe:              cocktails.RecipeResponseItem{result.Recipe.Steps},
	}, nil
}
