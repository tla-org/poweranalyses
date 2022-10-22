;  Compile with `emcc test.ll -o test.wasm`.

target triple = "wasm32-unknown-emscripten"

;  @ REPL[1]:1 within `f`
define i64 @julia_f_318(i64 signext %0) #0 {
top:
  ret i64 1
}

define i32 @main() #0 {
  call i32 @putchar(i32 72)
  call i32 @putchar(i32 101)
  call i32 @putchar(i32 108)
  call i32 @putchar(i32 108)
  call i32 @putchar(i32 111)
  call i32 @putchar(i32 10)
  ret i32 0
}

declare i32 @putchar(i32) #1
