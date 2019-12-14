package utils

func Check(e error) {
	if e != nil {
		panic(e)
	}
}

func Assert(condition bool, errorMessage string) {
	if !condition {
		panic("Assertion failed: " + errorMessage)
	}
}
