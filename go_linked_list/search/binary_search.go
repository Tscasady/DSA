package search

import (
	"fmt"
	"golang.org/x/exp/constraints"
)

type Both interface {
  constraints.Ordered
  comparable
}

func Binary_search[T Both](v T, arr []T) int {
  lo := 0
  high := len(arr)
  mid := (high - lo) / 2
  for v != arr[mid] {
    mid := (high - lo) / 2
    if v == arr[mid] {
      return mid
    } else if v < arr[mid] {
      high = mid
    } else {
      lo = mid + 1
    }
    fmt.Println(lo)
  }
  return mid
}

