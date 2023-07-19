package gen

import (
	"testing"

	"github.com/davecgh/go-spew/spew"
)

func TestName(t *testing.T) {
	spew.Dump(ImplRustFfi{}.Search(SearchRequest{
		Query:         "lyc",
		PageNumber:    10,
		ResultPerPage: 20,
	}))
}
