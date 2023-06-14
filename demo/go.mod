module github.com/andeya/fcplug/demo

go 1.20

replace (
	github.com/andeya/fcplug/demo/go_gen => ./go_gen
	github.com/andeya/fcplug/go/gocall => ../go/gocall
)

require (
	github.com/andeya/fcplug/go/gocall v0.0.0-00010101000000-000000000000
	github.com/andeya/gust v1.5.2
	github.com/davecgh/go-spew v1.1.1
	github.com/golang/protobuf v1.5.3
	github.com/google/flatbuffers v23.5.26+incompatible
)

require google.golang.org/protobuf v1.30.0 // indirect
