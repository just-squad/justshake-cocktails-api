package use_cases

import (
	"context"
	"github.com/google/uuid"
)

func New() *CocktailsUseCase {
	return &CocktailsUseCase{}
}

type CocktailsUseCase struct {
}

func (uc *CocktailsUseCase) GetById(ctx context.Context, request GetByIdRequest) (GetByIdResponse, error) {
	//if err != nil {
	//	return GetByIdResponse{}, fmt.Errorf("TranslationUseCase - History - s.repo.GetHistory: %w", err)
	//}
	return GetByIdResponse{
		Id:                  uuid.New(),
		Name:                "",
		RussianName:         "",
		History:             "",
		Tags:                nil,
		Tools:               nil,
		CompositionElements: nil,
		Recipe:              RecipeResponseItem{},
	}, nil
}

func (uc *CocktailsUseCase) GetByFilter(ctx context.Context, req GetByFilterRequest) (GetByFilterResponse, error) {
	return GetByFilterResponse{}, nil
}
