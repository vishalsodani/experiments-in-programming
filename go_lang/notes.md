imports not used leads to compile error
must use every variable declared - else compile error

goimports automatically adds import statements ( since it gets tedious to add)


can infer type of variable from initial value assigned

myVar := 1 //integer
myVar2 := 13.9 //float


multiple return values

language design - encourages to do explicitly check for errors

first class functions

annoymous functions

OOPS Like

anonymous struct 
	var bucket struct

custom types
	type Mybucket struct

can define methods on type
	func hi(Mybucket)

underlyting type is not a superclass; you dont inherity methods of that type
	type Mytype string

embed structs like inherited fields
methods from embed types are promoted as well as fields


use interfaces as new types
this makes Go a structural typing system; more stricter than duck typing
https://medium.com/higher-order-functions/duck-typing-vs-structural-typing-vs-nominal-typing-e0881860bf10


defer keyword defers function call till end of a function in which the func is called; even if an error occurs


golang errors are values; dont use panic as exceptions

