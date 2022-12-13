package main

import (
	"syscall/js"
	"strconv"
)

var Count = 0

func add(this js.Value,i []js.Value)interface{} {
	value1 := strconv.Itoa(1)
	int1, _ := strconv.Atoi(value1)
    js.Global().Set("output", Count+int1)
	Count = Count + int1
	println(Count) 
	document:=js.Global().Get("document")
	  document.Call("getElementById","int").Set("innerHTML",Count)
	// println(js.ValueOf(i[0].Int() + i[1].Int()).String())
	return Count
}

func subtract(this js.Value, i []js.Value)interface{} {
	value2 := strconv.Itoa(1)
	int2, _ := strconv.Atoi(value2)
    js.Global().Set("output", Count-int2)
	Count = Count - int2
	document:=js.Global().Get("document")
	  document.Call("getElementById","int").Set("innerHTML",Count)
    println(Count) 
	return Count
}

func reset(this js.Value, i []js.Value)interface{} {
	value3 := strconv.Itoa(0)
	int3, _ := strconv.Atoi(value3)
	Count =  int3
    js.Global().Set("output", Count)
	document:=js.Global().Get("document")
	  document.Call("getElementById","int").Set("innerHTML",Count)
    println(Count) 
	return Count
}

func registerCallbacks() {
    js.Global().Set("add", js.FuncOf(add))
    js.Global().Set("subtract", js.FuncOf(subtract))
	js.Global().Set("reset", js.FuncOf(reset))
}

func main() {
    c := make(chan struct{}, 0)

    println("WASM Go Initialized")
    // register functions
    registerCallbacks()
    <-c
}