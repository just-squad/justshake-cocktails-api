package use_cases

import (
	"context"
)

// Cocktails Интерфейсы
type (
	Cocktails interface {
		GetNames(ctx context.Context, req GetNamesRequest) (GetNamesResponse, error)
		GetById(ctx context.Context, req GetByIdRequest) (GetByIdResponse, error)
		GetByFilter(ctx context.Context, req GetByFilterRequest) (GetByFilterResponse, error)
	}
)
