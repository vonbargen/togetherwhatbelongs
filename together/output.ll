; ModuleID = 'Example'
source_filename = "Example"

@oberon_count = global i64 0
@oberon_offset = global i64 0
@oberon_points = global [10 x { double, double }] zeroinitializer
@.str = private constant [5 x i8] c"%lld\00"
@.str.1 = private constant [1 x i8] zeroinitializer

declare i32 @printf(ptr, ...)

declare i32 @puts(ptr)

define i64 @oberon_Add(i64 %0, i64 %1) {
entry:
  %a = alloca i64, align 8
  store i64 %0, ptr %a, align 4
  %b = alloca i64, align 8
  store i64 %1, ptr %b, align 4
  %load = load i64, ptr %a, align 4
  %load1 = load i64, ptr %b, align 4
  %add = add i64 %load, %load1
  ret i64 %add
}

define void @oberon_Init() {
entry:
  %i = alloca i64, align 8
  store i64 0, ptr @oberon_count, align 4
  store i64 0, ptr %i, align 4
  br label %forcond

forcond:                                          ; preds = %forincr, %entry
  %i1 = load i64, ptr %i, align 4
  %forcmp = icmp sle i64 %i1, 9
  br i1 %forcmp, label %forbody, label %forcont

forbody:                                          ; preds = %forcond
  %load = load i64, ptr %i, align 4
  %arrayidx = getelementptr [10 x { double, double }], ptr @oberon_points, i64 0, i64 %load
  %x = getelementptr inbounds { double, double }, ptr %arrayidx, i32 0, i32 0
  store double 0.000000e+00, ptr %x, align 8
  %load2 = load i64, ptr %i, align 4
  %arrayidx3 = getelementptr [10 x { double, double }], ptr @oberon_points, i64 0, i64 %load2
  %y = getelementptr inbounds { double, double }, ptr %arrayidx3, i32 0, i32 0
  store double 0.000000e+00, ptr %y, align 8
  br label %forincr

forincr:                                          ; preds = %forbody
  %current = load i64, ptr %i, align 4
  %next = add i64 %current, 1
  store i64 %next, ptr %i, align 4
  br label %forcond

forcont:                                          ; preds = %forcond
  ret void
}

define void @oberon_WriteInt(i64 %0) {
entry:
  %printf_call = call i32 (ptr, ...) @printf(ptr @.str, i64 %0)
  ret void
}

define void @oberon_WriteLn() {
entry:
  %puts_call = call i32 @puts(ptr @.str.1)
  ret void
}

define i32 @main() {
entry:
  call void @oberon_Init()
  store i64 100, ptr @oberon_offset, align 4
  %call = call i64 @oberon_Add(i64 5, i64 37)
  store i64 %call, ptr @oberon_count, align 4
  %load = load i64, ptr @oberon_count, align 4
  %load1 = load i64, ptr @oberon_offset, align 4
  %add = add i64 %load, %load1
  call void @oberon_WriteInt(i64 %add)
  call void @oberon_WriteLn()
  ret i32 0
}
