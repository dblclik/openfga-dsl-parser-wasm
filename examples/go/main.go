package main

import (
	"fmt"
	"io/ioutil"

	wasmer "github.com/wasmerio/wasmer-go/wasmer"
)

func main() {
	wasmBytes, _ := ioutil.ReadFile("../wasm_binaries/openfga_dsl_parser_wasm.wasm")

	engine := wasmer.NewEngine()

	// Create a Store
	store := wasmer.NewStore(engine)

	fmt.Println("Compiling module...")
	module, err := wasmer.NewModule(store, wasmBytes)

	if err != nil {
		fmt.Println("Failed to compile module:", err)
	}

	// Create an empty import object.
	importObject := wasmer.NewImportObject()

	fmt.Println("Instantiating module...")
	// Let's instantiate the Wasm module.
	instance, err := wasmer.NewInstance(module, importObject)
	defer instance.Close()

	if err != nil {
		panic(fmt.Sprintln("Failed to instantiate the module:", err))
	}

	// The module exports some utility functions, let's get them.
	//
	// These function will be used later in this example.

	memory, err := instance.Exports.GetMemory("memory")

	if err != nil {
		panic(fmt.Sprintln("Failed to get the `memory` memory:", err))
	}

	fmt.Println("Querying memory size...")
	size := memory.Size()
	fmt.Println("Memory size (pages):", size)
	fmt.Println("Memory size (pages as bytes):", size.ToBytes())
	fmt.Println("Memory size (bytes):", memory.DataSize())

	dsl_to_json, dsl_err := instance.Exports.GetFunction("dsl_to_json")
	if dsl_err != nil {
		fmt.Println("Got an error import dsl_to_json")
		return
	}
	doc_len, doc_len_err := instance.Exports.GetFunction("doc_len")
	if doc_len_err != nil {
		fmt.Println("Got an error import doc_len")
		return
	}

	// The DSL to parse
	dsl := []byte("type document")
	lengthOfDsl := len(dsl)

	// Allocate memory for the subject, and get a pointer to it.
	// Currently a fixed value, should be dynamic and support retries
	// if not empty
	inputPointer := 2200

	// Write the subject into the memory.
	input_memory := memory.Data()[inputPointer:]

	fmt.Println("WASM memory at pointer before write is ", input_memory[:lengthOfDsl])
	if !isEmpty(input_memory[:lengthOfDsl]) {
		fmt.Println("Target memory block is not empty!")
		return
	}

	for nth := 0; nth < lengthOfDsl; nth++ {
		input_memory[nth] = dsl[nth]
	}

	// C-string terminates by NULL.
	input_memory[lengthOfDsl] = 0

	fmt.Println("DSL as bytes is: ", dsl)
	fmt.Println("WASM memory at pointer after write is: ", input_memory[:lengthOfDsl])

	offset, parse_err := dsl_to_json(inputPointer, lengthOfDsl)
	if parse_err != nil {
		fmt.Println("There was an error parsing the dsl")
		return
	} else {
		fmt.Println("We have an offset...")
		resp_len, len_err := doc_len()
		if len_err != nil {
			fmt.Println("There was an error getting response length")
			return
		} else {
			fmt.Println("We have a response length...")
			result := memory.Data()[offset.(int32):(offset.(int32) + resp_len.(int32))]
			json := string(result)
			fmt.Println(result)
			fmt.Println(json)
		}
	}
}

func isEmpty(mem_slice []byte) bool {
	for _, mem_val := range mem_slice {
		if mem_val != 0 {
			return false
		}
	}
	return true
}
