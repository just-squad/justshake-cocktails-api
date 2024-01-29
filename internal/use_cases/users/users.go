package users

import (
	"context"
	"fmt"
	"github.com/google/uuid"
	user_aggregate "justshake/cocktails/internal/domain/user-aggregate"
	"slices"
)

type UseCase struct {
	usersRepo user_aggregate.UsersRepo
}

func New(cocktailsRepo user_aggregate.UsersRepo) *UseCase {
	return &UseCase{cocktailsRepo}
}

func (uc *UseCase) Create(ctx context.Context, req CreateUserRequest) error {
	user := user_aggregate.User{
		Id:                req.Id,
		TelegramId:        req.TelegramId,
		FavoriteCocktails: make([]uuid.UUID, 0),
	}
	return uc.usersRepo.Create(ctx, user)
}

func (uc *UseCase) Delete(ctx context.Context, req DeleteUserRequest) error {
	user, err := uc.usersRepo.GetByTelegramId(ctx, req.TelegramId)
	if err != nil {
		return err
	}
	return uc.usersRepo.Delete(ctx, user)
}

func (uc *UseCase) IsExist(ctx context.Context, req GetByTelegramIdRequest) (bool, error) {
	return uc.usersRepo.IsExist(ctx, req.Id)
}

func (uc *UseCase) GetByTelegramId(ctx context.Context, req GetByTelegramIdRequest) (UserResponse, error) {
	result, err := uc.usersRepo.GetByTelegramId(ctx, req.Id)
	if err != nil {
		return UserResponse{}, fmt.Errorf("CocktailsUseCase - GetById - uc.cocktailsRepo.GetById: %w", err)
	}
	return UserResponse{
		Id:                result.Id,
		TelegramId:        result.TelegramId,
		FavoriteCocktails: result.FavoriteCocktails,
	}, nil
}

func (uc *UseCase) AddCocktailToFavorite(ctx context.Context, req FavoriteCocktailRequest) error {
	user, err := uc.usersRepo.GetByTelegramId(ctx, req.TelegramId)
	if err != nil {
		return err
	}
	user.FavoriteCocktails = append(user.FavoriteCocktails, req.CocktailId)
	err = uc.usersRepo.Update(ctx, user)
	if err != nil {
		return err
	}
	return nil
}

func (uc *UseCase) RemoveCocktailFromFavorite(ctx context.Context, req FavoriteCocktailRequest) error {
	user, err := uc.usersRepo.GetByTelegramId(ctx, req.TelegramId)
	if err != nil {
		return err
	}
	indexOfCocktail := slices.Index(user.FavoriteCocktails, req.CocktailId)
	user.FavoriteCocktails = append(user.FavoriteCocktails[:indexOfCocktail], user.FavoriteCocktails[indexOfCocktail+1:]...)
	err = uc.usersRepo.Update(ctx, user)
	if err != nil {
		return err
	}
	return nil
}
