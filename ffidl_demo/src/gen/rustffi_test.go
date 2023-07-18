package gen

import "testing"

func TestName(t *testing.T) {
	t.Log(ImplRustFfi{}.GetUser("false"))
}
