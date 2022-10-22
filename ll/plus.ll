; ModuleID = 'text'
source_filename = "text"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "wasm32-unknown-emscripten"

define i64 @julia__(i64 signext %0, i64 signext %1) local_unnamed_addr #0 {
top:
  %2 = add i64 %1, %0
  ret i64 %2
}

attributes #0 = { "probe-stack"="inline-asm" }

!llvm.module.flags = !{!0, !1}

!0 = !{i32 2, !"Dwarf Version", i32 4}
!1 = !{i32 2, !"Debug Info Version", i32 3}

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
