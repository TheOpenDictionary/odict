package utils

// Map maps any array of type T to an array of type V
func Map[T any, V any](array []T, f func(T) V) []V {
	result := []V{}

	for _, item := range array {
		result = append(result, f(item))
	}

	return result
}
