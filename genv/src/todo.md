math.random

返回伪随机值。该函数将为每个脚本执行生成不同的值序列。对可选的seed参数使用相同的值将产生可重复的序列。
math.random(min, max, seed) → series float
返回值
一个随机值。
参数
min (series int/float) 随机值范围的下限。该值不包括在范围内。默认值为0。
max (series int/float) 随机值范围的上限。该值不包括在范围内。默认值为1。
seed (input int) 可选参数。当使用相同的seed时，允许连续调用该函数以产生一组可重复的值。





Pine脚本语言参考手册
运算子
!=

不等于。适用于任何类型的表达式。
expr1 != expr2
返回值
布尔值，或一系列布尔值。
%

模数(整数余数)。 适用于数值表达式。
expr1 % expr2
返回值
整数或浮点值，或一系列值。
备注
在Pine Script™，当计算整数余数时，商会被截断，即朝最低绝对值四舍五入。结果值将与被除数具有相同的符号。
示例：-1 % 9 = -1 - 9 * truncate(-1/9) = -1 - 9 * truncate(-0.111) = -1 - 9 * 0 = -1。
%=

模数指派。适用于数值表达式。
expr1 %= expr2
例子
//@version=5
indicator("%=")
// Equals to expr1 = expr1 % expr2.
a = 3
b = 3
a %= b
// Result: a = 0.
plot(a)
返回值
整数或浮点值，或一系列值。
*

乘法。适用于数值表达式。
expr1 * expr2
返回值
整数或浮点值，或一系列值。
*=

乘法指派。适用于数值表达式。
expr1 *= expr2
例子
//@version=5
indicator("*=")
// Equals to expr1 = expr1 * expr2.
a = 2
b = 3
a *= b
// Result: a = 6.
plot(a)
返回值
整数或浮点值，或一系列值。
+

添加或一元正号。适用于数值表达式或字符串。
expr1 + expr2
+ expr
  返回值
  字符串的二进制`+`返回expr1和expr2的合并
  数字返回整数或浮点值，或一系列值：
  二进制'+'返回expr1加expr2。
  一元“+”返回expr(对一元运算符对称不添加任何内容)。
  备注
  您可以使用带数字的算术运算符以及变量数列。 在使用数列的情况下，操作符应用于元素。
  +=

加法指派。适用于数值表达式或字符串。
expr1 += expr2
例子
//@version=5
indicator("+=")
// Equals to expr1 = expr1 + expr2.
a = 2
b = 3
a += b
// Result: a = 5.
plot(a)
返回值
对于字符串，返回expr1和expr2的串联。对于数字，返回整数或浮点值，或一系列值。
备注
您可以使用带数字的算术运算符以及变量数列。 在使用数列的情况下，操作符应用于元素。
-

减法或一元负号。 适用于数值表达式。
expr1 - expr2
- expr
  返回值
  返回整数或浮点值，或一系列值：
  二进制'+'返回expr1减expr2。
  一元的`-`返回expr的否定式。
  备注
  您可以使用带数字的算术运算符以及变量数列。 在使用数列的情况下，操作符应用于元素。
  -=

减法指派。适用于数值表达式。
expr1 -= expr2
例子
//@version=5
indicator("-=")
// Equals to expr1 = expr1 - expr2.
a = 2
b = 3
a -= b
// Result: a = -1.
plot(a)
返回值
整数或浮点值，或一系列值。
/

除法。适用于数值表达式。
expr1 / expr2
返回值
整数或浮点值，或一系列值。
/=

除法指派。适用于数值表达式。
expr1 /= expr2
例子
//@version=5
indicator("/=")
// Equals to expr1 = expr1 / expr2.
a = 3
b = 3
a /= b
// Result: a = 1.
plot(a)
返回值
整数或浮点值，或一系列值。
<

小于。适用于数值表达式。
expr1 < expr2
返回值
布尔值，或一系列布尔值。
<=

小于或等于。适用于数值表达式。
expr1 <= expr2
返回值
布尔值，或一系列布尔值。
==

等于。 适用于任何类型的表达。
expr1 == expr2
返回值
布尔值，或一系列布尔值。
=>

'=>'运算符用于用户定义的函数声明和switch语句中。
函数声明语法是：
<identifier>([<parameter_name>[=<default_value>]], ...) =>  
<local_block>  
<function_result>
是零个或多个Pine Script™语句。
是一个变量、一个表达式或一个元组。
例子
//@version=5
indicator("=>")
// single-line function
f1(x, y) => x + y
// multi-line function
f2(x, y) =>
sum = x + y
sumChange = ta.change(sum, 10)
// Function automatically returns the last expression used in it
plot(f1(30, 8) + f2(1, 3))
备注
您可以在用户手册的声明函数和脚本库页面中了解有关用户定义函数的更多信息。
>

大于。适用于数值表达式。
expr1 > expr2
返回值
布尔值，或一系列布尔值。
>=

大于或等于。适用于数值表达式。
expr1 >= expr2
返回值
布尔值，或一系列布尔值。
?:

三元条件运算符。
expr1 ? expr2 : expr3
例子
//@version=5
indicator("?:")
// Draw circles at the bars where open crosses close
s2 = ta.cross(open, close) ? math.avg(open,close) : na
plot(s2, style=plot.style_circles, linewidth=2, color=color.red)

// Combination of ?: operators for 'switch'-like logic
c = timeframe.isintraday ? color.red : timeframe.isdaily ? color.green : timeframe.isweekly ? color.blue : color.gray
plot(hl2, color=c)
返回值
如果expr1被评估为true，则expr2，否则为expr3。 零值(0和NaN，+ Infinity，-Infinity)被视为false，其他值皆为true。
备注
如果您不需要，请使用na作为“else”分支。
您可以结合使用两个或多个?:运算符，以实现类似于“switch”的语句（请参见上面的示例）。
您可以使用带数字的算术运算符以及变量数列。 在使用数列的情况下，操作符应用于元素。
另见
na
[]

系列下标。 提供对expr1系列的以前值的访问。 expr2是过去k线的数目，必须是数值。 浮动将被向下舍入。
expr1[expr2]
例子
//@version=5
indicator("[]")
// [] can be used to "save" variable value between bars
a = 0.0 // declare `a`
a := a[1] // immediately set current value to the same as previous. `na` in the beginning of history
if high == low // if some condition - change `a` value to another
a := low
plot(a)
返回值
一系列数值。
另见
math.floor
and

逻辑 AND。适用于布尔表达式。
expr1 and expr2
返回值
布尔值，或一系列布尔值。
array

用于显式声明变量或参数的"array"类型的关键字。可以使用array.new、array.from函数创建阵列对象（或ID）。
例子
//@version=5
indicator("array", overlay=true)
array<float> a = na
a := array.new<float>(1, close)
plot(array.get(a, 0))
备注
阵列对象总是“系列”形式。
另见
var
line
label
table
box
array.new<type>
array.from
bool

用于显式声明变量或参数的“bool”（布尔）类型的关键字。"Bool"变量的值可以是true、false或na。
例子
//@version=5
indicator("bool")
bool b = true    // Same as `b = true`
b := na
plot(b ? open : close)
备注
在变量声明中明确提及类型是可选的，除非它是用na初始化的。在 类型系统的用户手册页面中了解有关Pine Script™类型的更多信息。
另见
var
varip
int
float
color
string
true
false
box

用于显式声明变量或参数的"box"类型的关键字。可以使用box.new函数创建box对象（或ID）。
例子
//@version=5
indicator("box")
// Empty `box1` box ID.
var box box1 = na
// `box` type is unnecessary because `box.new()` returns a "box" type.
var box2 = box.new(na, na, na, na)
box3 = box.new(time, open, time + 60 * 60 * 24, close, xloc=xloc.bar_time)
备注
box对象总是"series"形式。
另见
var
line
label
table
box.new
color

用于显式声明变量或参数的"color"类型的关键字。
例子
//@version=5
indicator("color", overlay = true)

color textColor = color.green   
color labelColor = #FF000080  // Red color (FF0000) with 50% transparency (80 which is half of FF).
if barstate.islastconfirmedhistory
label.new(bar_index, high, text = "Label", color = labelColor, textcolor = textColor)

// When declaring variables with color literals, built-in constants(color.green) or functions (color.new(), color.rgb()), the "color" keyword for the type can be omitted.
c = color.rgb(0,255,0,0)
plot(close, color = c)
备注
颜色文字具有以下格式：#RRGGBB 或 #RRGGBBAA。 字母对代表00到FF的十六进制值（十进制的0到255），其中RR、GG和BB对是颜色的红色、绿色和蓝色分量的值。AA是颜色透明度（或alpha分量）的可选值，其中00不可见，FF不透明。 当没有提供AA对时，使用FF。十六进制字母可以是大写或小写。
在变量声明中明确提及类型是可选的，除非它是用na初始化的。在 类型系统的用户手册页面中了解有关Pine Script™类型的更多信息。
另见
var
varip
int
float
string
color.rgb
color.new
export

在库中用于函数声明或用户定义的类型定义的前缀，这些定义可从导入库的其它脚本中获得。
例子
//@version=5
//@description Library of debugging functions.
library("Debugging_library", overlay = true)
//@function Displays a string as a table cell for debugging purposes.
//@param txt String to display.
//@returns Void.
export print(string txt) =>
var table t = table.new(position.middle_right, 1, 1)
table.cell(t, 0, 0, txt, bgcolor = color.yellow)
// Using the function from inside the library to show an example on the published chart.
// This has no impact on scripts using the library.
print("Library Test")
备注
每个库必须至少有一个导出函数或用户定义类型(UDT)。
如果导出的函数是数组、可变变量（使用 := 重新分配）或'input'形式的变量，则它们不能使用全局范围内的变量。
导出的函数不能使用 `request.*()` 函数。
导出的函数必须显式声明每个参数的类型，并且所有参数都必须在函数体中使用。默认情况下，传递给导出函数的所有参数都是series形式，除非它们在函数的签名中明确指定为simple。
@description、@function、@param、@type、@field 和@returns编译器注释用于自动生成库的描述和发行说明，以及Pine Script™编辑器的工具提示。
另见
library
false

表示bool值的文字，以及比较操作的结果。
备注
请参阅比较运算符和逻辑运算符的用户手册。
另见
bool
float

用于显式声明变量或参数的“float”（浮点）类型的关键字。
例子
//@version=5
indicator("float")
float f = 3.14    // Same as `f = 3.14`
f := na
plot(f)
备注
在变量声明中明确提及类型是可选的，除非它是用na初始化的。在 类型系统的用户手册页面中了解有关Pine Script™类型的更多信息。
另见
var
varip
int
bool
color
string
for

'for'结构允许重复执行多个语句：
[var_declaration =] for counter = from_num to to_num [by step_num]  
statements | continue | break  
return_expression
var_declaration - 一个可选的变数声明，它将被指派为回圈的 return_expression 的值。
counter - 保存回圈计数器值的变数，在回圈的每次迭代中递增/递减 1 或 step_num 值。
from_num - 计数器的起始值。允许使用“series int/float”值/表达式。
to_num - 计数器的最终值。当计数器大于to_num（或小于to_num在from_num > to_num的情况下）时，循环中断。允许使用“series int/float”值/表达式，但它们仅在循环的第一次迭代时进行评估。
step_num - 计数器的递增/递减值。它是可选的。默认值为+1或-1，具体取决于from_num或to_num中最大的一个。使用值时，计数器也会根据from_num或to_num中最大的那个而递增/递减，因此step_num的+/-符号是可选的。
statements | continue | break - 任意数量的语句，或'continue'或'break'关键字，缩进4个空格或一次 tab。
return_expression - 循环的返回值，如果存在，则分配给var_declaration中的变量。 如果循环由于“continue”或“break”关键字而退出，则循环的返回值是在循环退出之前分配值的最后一个变量的返回值。
continue - 只能在回圈中使用的关键字。它导致回圈的下一次迭代被执行。
break - 退出回圈的关键字。
例子
//@version=5
indicator("for")
// Here, we count the quantity of bars in a given 'lookback' length which closed above the current bar's close
qtyOfHigherCloses(lookback) =>
int result = 0
for i = 1 to lookback
if close[i] > close
result += 1
result
plot(qtyOfHigherCloses(14))
另见
for...in
for...in

`for...in` 结构允许为数组中的每个元素重复执行多个语句。它可以与任一参数一起使用：`array_element`，或与两个参数一起使用：`[index, array_element]`。 第二种形式不影响循环的功能。它在元组的第一个变量中跟踪当前迭代的索引。
[var_declaration =] for array_element in array_id  
statements | continue | break  
return_expression

[var_declaration =] for [index, array_element] in array_id  
statements | continue | break  
return_expression
var_declaration - 一个可选的变量声明，将被赋予循环的 `return_expression` 的值。
索引 - 跟踪当前迭代索引的可选变量。索引从 0 开始。变量在循环体中是不可变的。使用时，它必须包含在一个也包含 `array_element` 的元组中。
array_element - 包含要在循环中处理的每个连续阵列元素的变量。该变量在循环体中是不可变的。
array_id - 回圈迭代的阵列ID。
statements | continue | break - 任意数量的语句，或'continue'或'break'关键字，缩进4个空格或一次 tab。
return_expression - 循环的返回值分配给 `var_declaration` 中的变量，如果存在的话。 如果循环由于'continue'或'break'关键字而退出，则循环的返回值是循环退出前最后一个赋值的变量。
continue - 只能在回圈中使用的关键字。它导致回圈的下一次迭代被执行。
break - 退出回圈的关键字。
允许在循环内修改阵列的元素或其大小。
在这里，我们使用 `for...in` 的单参数形式来确定在每个K线上，有多少K线的OHLC值大于'close'值的SMA：
例子
//@version=5
indicator("for...in")
// Here we determine on each bar how many of the bar's OHLC values are greater than the SMA of 'close' values
float[] ohlcValues = array.from(open, high, low, close)
qtyGreaterThan(value, array) =>
int result = 0
for currentElement in array
if currentElement > value
result += 1
result
plot(qtyGreaterThan(ta.sma(close, 20), ohlcValues))
在这里，我们使用for...in的两个参数形式将我们的 `isPos` 数组的值设置为 `true`，当它们在我们的 `valuesArray` 数组中的对应值为正时：
例子
//@version=5
indicator("for...in")
var valuesArray = array.from(4, -8, 11, 78, -16, 34, 7, 99, 0, 55)
var isPos = array.new_bool(10, false)

for [index, value] in valuesArray
if value > 0
array.set(isPos, index, true)

if barstate.islastconfirmedhistory
label.new(bar_index, high, str.tostring(isPos))
将矩阵行作为阵列进行迭代。
例子
//@version=5
indicator("`for ... in` matrix Example")

// Create a 2x3 matrix with values `4`.
matrix1 = matrix.new<int>(2, 3, 4)

sum = 0.0
// Loop through every row of the matrix.
for rowArray in matrix1
// Sum values of the every row
sum += array.sum(rowArray)

plot(sum)
另见
for
if

If语句定义了在满足表达式条件时必须执行的语句块。
要访问和使用 if 语句，应在第一行代码中指定Pine Script™语言的版本 >= 2，例如：//@version=5
Pine Script™语言的第4版允许您使用“else if”语法。
通用编码来自：
var_declarationX = if condition  
var_decl_then0  
var_decl_then1  
…  
var_decl_thenN  
else if [optional block]  
var_decl_else0  
var_decl_else1  
…  
var_decl_elseN  
else  
var_decl_else0  
var_decl_else1  
…  
var_decl_elseN  
return_expression_else
在哪
var_declarationX - 此变量获取if语句的值
条件 — 如果条件为true，则使用block ‘then’中的逻辑(var_decl_then0, var_decl_then1, 等)。
如果条件为false，则使用block 'else'中的逻辑(var_decl_else0, var_decl_else1, 等)。
return_expression_then **，** return_expression_else - 模块中的最后一个表达式或者来自块else的表达式将返回语句的最终值。 如果变量的声明在最后，它的值将是结果值。
if语句的返回值的类型取决于return_expression_then和return_expression_else类型（它们的类型必须匹配：从那时起，当你在else块中有一个字符串值时，不可能返回一个整数值）。
例子
//@version=5
indicator("if")
// This code compiles
x = if close > open
close
else
open

// This code doesn’t compile
// y = if close > open
//     close
// else
//     "open"
plot(x)
可以省略`else`块。在这种情况下，如果条件为false，则会为var_declarationX变量分配一个“empty”值（na、false 或“”）：
例子
//@version=5
indicator("if")
x = if close > open
close
// If current close > current open, then x = close.
// Otherwise the x = na.
plot(x)
可以使用多个“else if”块或根本不使用。“then”、“else if”、“else”块被移动了四个空格：
例子
//@version=5
indicator("if")
x = if open > close
5
else if high > low
close
else
open
plot(x)
可以忽略`if`语句的结果值（“var_declarationX=”可以省略）。如果您需要表达式的副作用，它可能很有用，例如在策略交易中：
例子
//@version=5
strategy("if")
if (ta.crossover(high, low))
strategy.entry("BBandLE", strategy.long, stop=low, oca_name="BollingerBands", oca_type=strategy.oca.cancel, comment="BBandLE")
else
strategy.cancel(id="BBandLE")
If语句可以相互包含：
例子
//@version=5
indicator("if")
float x = na
if close > open
if close > close[1]
x := close
else
x := close[1]
else
x := open
plot(x)
import

用于将外部library加载到脚本中并将其函数绑定到命名空间。导入脚本可以是指标、策略或其他库。库必须先发布（私密或公开），然后才能导入。
import {username}/{libraryName}/{libraryVersion} as {alias}
例子
//@version=5
indicator("num_methods import")
// Import the first version of the username’s "num_methods" library and assign it to the "m" namespace",
import username/num_methods/1 as m
// Call the “sinh()” function from the imported library
y = m.sinh(3.14)
// Plot value returned by the "sinh()" function",
plot(y)
参数
username (literal string) 脚本库作者的用户名。
libraryName (literal string) 导入脚本库的名称，对应于作者在其库脚本中使用的`title`参数。
libraryVersion (literal int) 导入脚本库的版本号。
alias (literal string) 用于引用库函数的命名空间。可选。默认值为libraryName字符串。
备注
允许使用替代内置命名空间，如 math.* 或 strategy.* 的别名，但如果库包含影响Pine Script™内置函数的函数名称，则内置函数将不可用。同一版本的库只能导入一次。每个导入的库的别名必须不同。调用库函数时，不允许将其参数转换为声明类型以外的类型。
另见
library
export
int

用于显式声明变量或参数的“int”（整数）类型的关键字。
例子
//@version=5
indicator("int")
int i = 14    // Same as `i = 14`
i := na
plot(i)
备注
在变量声明中明确提及类型是可选的，除非它是用na初始化的。在 类型系统的用户手册页面中了解有关Pine Script™类型的更多信息。
另见
var
varip
float
bool
color
string
label

用于显式声明变量或参数的"label"类型的关键字。可以使用label.new函数创建标签对象（或ID）。
例子
//@version=5
indicator("label")
// Empty `label1` label ID.
var label label1 = na
// `label` type is unnecessary because `label.new()` returns "label" type.
var label2 = label.new(na, na, na)
if barstate.islastconfirmedhistory
label3 = label.new(bar_index, high, text = "label3 text")
备注
标签对象总是"series"形式。
另见
var
line
box
label.new
line

用于显式声明变量或参数的"line"类型的关键字。可以使用line.new函数创建线条对象（或ID）。
例子
//@version=5
indicator("line")
// Empty `line1` line ID.
var line line1 = na
// `line` type is unnecessary because `line.new()` returns "line" type.
var line2 = line.new(na, na, na, na)
line3 = line.new(bar_index - 1, high, bar_index, high, extend = extend.right)
备注
线条对象总是"series"形式。
另见
var
label
box
line.new
linefill

用于显式声明变量或参数的“linefill”类型的关键字。可以使用linefill.new函数创建行linefill对象（或ID）。
例子
//@version=5
indicator("linefill", overlay=true)
// Empty `linefill1` line ID.
var linefill linefill1 = na
// `linefill` type is unnecessary because `linefill.new()` returns "linefill" type.
var linefill2 = linefill.new(na, na, na)

if barstate.islastconfirmedhistory
line1 = line.new(bar_index - 10, high+1, bar_index, high+1, extend = extend.right)
line2 = line.new(bar_index - 10, low+1, bar_index, low+1, extend = extend.right)
linefill3 = linefill.new(line1, line2, color = color.new(color.green, 80))
备注
Linefill对象始终是"series"形式。
另见
var
line
label
table
box
linefill.new
matrix

用于显式声明变量或参数的"matrix"类型的关键字。可以使用matrix.new 函数创建矩阵对象（或 ID）。
例子
//@version=5
indicator("matrix example")

// Create `m1` matrix of `int` type.
matrix<int> m1 = matrix.new<int>(2, 3, 0)

// `matrix<int>` is unnecessary because the `matrix.new<int>()` function returns an `int` type matrix object.
m2 = matrix.new<int>(2, 3, 0)

// Display matrix using a label.
if barstate.islastconfirmedhistory
label.new(bar_index, high, str.tostring(m2))
备注
矩阵对象总是“系列”形式。
另见
var
matrix.new<type>
array
not

逻辑求反(NOT)。 适用于布尔表达式。
not expr1
返回值
布尔值，或一系列布尔值。
or

逻辑 OR。适用于布尔表达式。
expr1 or expr2
返回值
布尔值，或一系列布尔值。
series

series 是一个关键字，可在脚本库的导出函数中用于指定函数参数所需的类型形式。显式使用 `series` 关键字通常是不必要的，因为默认情况下，导出函数的所有参数都会自动转换为“series”形式。
export <functionName>([[series] <type>] <arg1>[ = <default_value>])
例子
//@version=5
//@description Library of debugging functions.
library("Debugging_library", overlay = true)
export smaCustom(series float source, series int length) =>
ta.sma(source, length)
simple

simple 是一个关键字，可以在脚本库的导出函数中使用，以指定函数参数所需的类型形式。 默认情况下，导出函数的所有参数都会自动转换为“series”类型的形式。在某些情况下，这会使参数无法与不支持“series”形式的内置函数一起使用。对于这些情况，可以使用 `simple` 关键字。
export <functionName>([[simple] <type>] <arg1>[ = <default_value>])
例子
//@version=5
//@description Library of debugging functions.
library("Debugging_library", overlay = true)
export emaWrong(float source, int length) =>
// By default, both `source` and `length` will expect values of the `series` type form: `series float` for `source`, `series int` for `length`.
// This function will not compile because `ema()` does not support a "series int" argument for `length`. A "simple int" is required.
ta.ema(source, length)

export emaRight(float source, simple int length) =>
// This function requires an argument of "simple int" type for its `length` parameter.
ta.ema(source, length)
string

用于显式声明变量或参数的"string"类型的关键字。
例子
//@version=5
indicator("string")
string s = "Hello World!"    // Same as `s = "Hello world!"`
// string s = na // same as ""
plot(na, title=s)
备注
在变量声明中明确提及类型是可选的，除非它是用na初始化的。在 类型系统的用户手册页面中了解有关Pine Script™类型的更多信息。
另见
var
varip
int
float
bool
str.tostring
str.format
switch

switch运算符根据条件和表达式的值将控制权转移到几个语句之一。
[variable_declaration = ] switch expression  
value1 => local_block  
value2 => local_block  
…  
=> default_local_block

[variable_declaration = ] switch  
boolean_expression1 => local_block  
boolean_expression2 => local_block  
…  
=> default_local_block
带表达式的switch：
例子
//@version=5
indicator("Switch using an expression")

string i_maType = input.string("EMA", "MA type", options = ["EMA", "SMA", "RMA", "WMA"])

float ma = switch i_maType
"EMA" => ta.ema(close, 10)
"SMA" => ta.sma(close, 10)
"RMA" => ta.rma(close, 10)
// Default used when the three first cases do not match.
=> ta.wma(close, 10)

plot(ma)
不带表达式的switch：
例子
//@version=5
strategy("Switch without an expression", overlay = true)

bool longCondition  = ta.crossover( ta.sma(close, 14), ta.sma(close, 28))
bool shortCondition = ta.crossunder(ta.sma(close, 14), ta.sma(close, 28))

switch
longCondition  => strategy.entry("Long ID", strategy.long)
shortCondition => strategy.entry("Short ID", strategy.short)
返回值
执行的本地语句块中最后一个表达式的值。
备注
只能执行`local_block`实例或`default_local_block`之一。`default_local_block`仅与`=>`标记一起引入，并且仅在没有执行前面的块时才执行。如果`switch`语句的结果被分配给一个变量并且没有指定`default_local_block`，如果没有执行`local_block`，则该语句返回`na`。将`switch`语句的结果分配给变量时，所有`local_block`实例必须返回相同类型的值。
另见
if
?:
table

用于显式声明变量或参数的"table"类型的关键字。可以使用table.new函数创建表格对象（或ID）。
例子
//@version=5
indicator("table")
// Empty `table1` table ID.
var table table1 = na
// `table` type is unnecessary because `table.new()` returns "table" type.
var table2 = table.new(position.top_left, na, na)

if barstate.islastconfirmedhistory
var table3 = table.new(position = position.top_right, columns = 1, rows = 1, bgcolor = color.yellow, border_width = 1)
table.cell(table_id = table3, column = 0, row = 0, text = "table3 text")
备注
表格对象总是"series"形式。
另见
var
line
label
box
table.new
true

表示bool变量可以保存的值之一的文字，或者当表达式使用比较或逻辑运算符时可以计算的值。
备注
请参阅比较运算符和逻辑运算符的用户手册。
另见
bool
type

此关键字允许您声明可以从中创建对象的用户定义类型(UDT)。 UDT是复合类型；它们包含任意数量的字段，可以是任何类型，包括正在定义的UDT。 定义UDT的语法是：
[export ]type <UDT_identifier>
<field_type> <field_name> [= <value>]
…
一旦定义了UDT，就可以使用 UDT_identifier.new() 构造从中创建对象。创建新对象时，如果在UDT的定义中指定了一个，则其字段将使用默认值进行初始化，否则使用na进行初始化。或者，可以在创建对象时指定字段值，方法是将它们作为参数包含在“*.new()”调用中，如 `newFooObject = foo.new(x = true)`，其中“foo”是先前定义的UDT的名称，“x”是其“bool”类型的字段之一。
有关详细信息，请参阅用户手册中关于定义 UDT和使用对象的部分。
UDT可以从库中导出。请参阅脚本库上的用户手册页面。
例子
//@version=5
indicator("Multi Time Period Chart", overlay = true)

timeframeInput = input.timeframe("1D")

type bar
float o = open
float h = high
float l = low
float c = close
int   t = time

drawBox(bar b, right) =>
bar s = bar.new()
color boxColor = b.c >= b.o ? color.green : color.red
box.new(b.t, b.h, right, b.l, boxColor, xloc = xloc.bar_time, bgcolor = color.new(boxColor, 90))

updateBox(box boxId, bar b) =>
color boxColor = b.c >= b.o ? color.green : color.red
box.set_border_color(boxId, boxColor)
box.set_bgcolor(boxId, color.new(boxColor, 90))
box.set_top(boxId, b.h)
box.set_bottom(boxId, b.l)
box.set_right(boxId, time)

secBar = request.security(syminfo.tickerid, timeframeInput, bar.new())

if not na(secBar)
// To avoid a runtime error, only process data when an object exists.
if not barstate.islast
if timeframe.change(timeframeInput)
// On historical bars, draw a new box in the past when the HTF closes.
drawBox(secBar, time[1])
else
var box lastBox = na
if na(lastBox) or timeframe.change(timeframeInput)
// On the last bar, only draw a new current box the first time we get there or when HTF changes.
lastBox := drawBox(secBar, time)
else
// On other chart updates, use setters to modify the current box.
updateBox(lastBox, secBar)
var

var 是用于分配和一次性初始化变量的关键字。
通常，不包含关键字var的变量赋值语法会导致每次更新数据时都会覆盖变量的值。 与此相反，当使用关键字var分配变量时，尽管数据更新，它们仍可以“保持状态”，只有在满足if-expressions中的条件时才更改它。
var variable_name = expression
哪里：
variable_name - Pine Script™中允许的用户变量的任何名称（可以包含大写和小写拉丁字符、数字和下划线(_)，但不能以数字开头）。
expression - 任何算术表达式，就像定义常规变量一样。 将计算表达式并将其分配给变量一次。
例子
//@version=5
indicator("Var keyword example")
var a = close
var b = 0.0
var c = 0.0
var green_bars_count = 0
if close > open
var x = close
b := x
green_bars_count := green_bars_count + 1
if green_bars_count >= 10
var y = close
c := y
plot(a)
plot(b)
plot(c)
变量 'a' 保持系列中每根K线的第一根K线的收盘价。
变量 'b' 保持系列中第一根“绿色”K线的收盘价。
变量 'c' 保持系列中第十根“绿色”K线的收盘价。
varip

varip（var intrabar persist）是用于分配和一次性初始化变量的关键词。它与var关键词相似，但是使用varip声明的变量在实时K线更新之间保留其值。
varip variable_name = expression
哪里：
variable_name - Pine Script™中允许的用户变量的任何名称（可以包含大写和小写拉丁字符、数字和下划线(_)，但不能以数字开头）。
expression - 任何算术表达式，就像定义常规变量时一样。在第一根K线上，表达式仅计算一次并将其分配给变量一次。
例子
//@version=5
indicator("varip")
varip int v = -1
v := v + 1
plot(v)
使用var时，绘图将返回bar_index的值。使用varip，在历史K线上会发生相同的行为，但是在实时K线上，该图将返回一个值，该值对于每一tick都增加一。
备注
只能与简单类型，例如float、int、bool、string，和这些类型的阵列一起使用。
while

`while`语句允许本地代码块的条件迭代。
variable_declaration = while boolean_expression  
…  
continue  
…  
break  
…  
return_expression
哪里：
variable_declaration - 可选的变量声明。`return expression`可以为这个变量提供初始化值。
boolean_expression - 如果为true，则执行`while`语句的本地块。如果为false，则在`while`语句之后继续执行脚本。
continue - `continue` 关键字导致循环分支到下一次迭代。
break - `break` 关键字导致循环终止。脚本的执行在 `while` 语句之后恢复。
return_expression - 提供 `while` 语句返回值的可选行。
例子
//@version=5
indicator("while")
// This is a simple example of calculating a factorial using a while loop.
int i_n = input.int(10, "Factorial Size", minval=0)
int counter   = i_n
int factorial = 1
while counter > 0
factorial := factorial * counter
counter   := counter - 1

plot(factorial)
备注
初始 `while` 行之后的本地代码块必须缩进四个空格或一个制表符。要终止 `while` 循环，`while` 后面的布尔表达式必须最终变为 false，或者必须执行 `break`。
内置变量
adjustment.dividends

常数股息调整（应用股息调整）。
类型
const string
另见
adjustment.none
adjustment.splits
ticker.new
adjustment.none

无调整类型的常量（不应用调整）。
类型
const string
另见
adjustment.splits
adjustment.dividends
ticker.new
adjustment.splits

分割调整类型的常量（应用分割调整）。
类型
const string
另见
adjustment.none
adjustment.dividends
ticker.new
alert.freq_all

与alert()函数的'freq'参数一起使用的命名常数。
所有函数调用触发警报。
类型
const string
另见
alert
alert.freq_once_per_bar

与alert()函数的'freq'参数一起使用的命名常数。
K线中的第一个函数调用触发警报。
类型
const string
另见
alert
alert.freq_once_per_bar_close

与alert()函数的'freq'参数一起使用的命名常数。
该函数调用仅在实时K线的最后一个脚本迭代期间发生时，在关闭时触发警报。
类型
const string
另见
alert
bar_index

目前的价格棒指数。 编号从零开始，第一个条的索引为0。
类型
series int
例子
//@version=5
indicator("bar_index")
plot(bar_index)
plot(bar_index > 5000 ? close : 0)
备注
请注意， bar_index **已替换版本4中的** n 变量。
请注意，K线索引从第一根历史K线起算为0。
请注意，使用此变量/函数可能会导致指标重新绘制。
另见
last_bar_index
barstate.isfirst
barstate.islast
barstate.isrealtime
barmerge.gaps_off

合并所请求数据的策略。 数据不间断地合并，所有的差距都以先前最近的现有值填满。
类型
barmerge_gaps
另见
request.security
barmerge.gaps_on
barmerge.gaps_on

给所请求的数据合并策略。 数据与可能的差距(na值)合并。
类型
barmerge_gaps
另见
request.security
barmerge.gaps_off
barmerge.lookahead_off

合并所请求数据位置的策略。 所请求的条形图与当前的条形图按照k线收盘时间合并。 这种合并策略禁止从“未来”获取数据计算历史的影响。
类型
barmerge_lookahead
另见
request.security
barmerge.lookahead_on
barmerge.lookahead_on

合并所请求数据位置的策略。 请求的条形图与当前的条形图按照k线开盘时间合并。 这种合并策略可能导致从“未来”获取数据计算历史的不良影响。 这在回溯测试策略中不被接受，但在指标中可使用。
类型
barmerge_lookahead
另见
request.security
barmerge.lookahead_off
barstate.isconfirmed

如果脚本正在计算当前k线的最后(关闭)更新，则返回true。 下一个脚本将在新K线数据上计算。
类型
series bool
备注
使用此变量的PineScript代码可以对历史记录和实时数据进行不同的计算。
不建议在request.security表达式中使用barstate.isconfirmed。它从request.security请求的值是不可预测的。
请注意，使用此变量/函数可能会导致指标重新绘制。
另见
barstate.isfirst
barstate.islast
barstate.ishistory
barstate.isrealtime
barstate.isnew
barstate.islastconfirmedhistory
barstate.isfirst

如果当前k线为k线组的第一条k线，则返回true，否则返回false。
类型
series bool
备注
使用此变量的PineScript代码可以对历史记录和实时数据进行不同的计算。
请注意，使用此变量/函数可能会导致指标重新绘制。
另见
barstate.islast
barstate.ishistory
barstate.isrealtime
barstate.isnew
barstate.isconfirmed
barstate.islastconfirmedhistory
barstate.ishistory

如果当前k线为历史k线，则返回true，否则返回false。
类型
series bool
备注
使用此变量的PineScript代码可以对历史记录和实时数据进行不同的计算。
请注意，使用此变量/函数可能会导致指标重新绘制。
另见
barstate.isfirst
barstate.islast
barstate.isrealtime
barstate.isnew
barstate.isconfirmed
barstate.islastconfirmedhistory
barstate.islast

如果当前k线为k线组的最后一条k线，则返回true，否则返回false。
类型
series bool
备注
使用此变量的PineScript代码可以对历史记录和实时数据进行不同的计算。
请注意，使用此变量/函数可能会导致指标重新绘制。
另见
barstate.isfirst
barstate.ishistory
barstate.isrealtime
barstate.isnew
barstate.isconfirmed
barstate.islastconfirmedhistory
barstate.islastconfirmedhistory

如果市场收盘时脚本在数据集的最后一根K线上执行，或者脚本正在实时K线之前的K线上执行，如果市场开盘，则返回true。否则返回false。
类型
series bool
备注
使用此变量的PineScript代码可以对历史记录和实时数据进行不同的计算。
请注意，使用此变量/函数可能会导致指标重新绘制。
另见
barstate.isfirst
barstate.islast
barstate.ishistory
barstate.isrealtime
barstate.isnew
barstate.isnew

如果脚本目前在新k线上计算着，则返回true，否则返回false。
类型
series bool
备注
使用此变量的PineScript代码可以对历史记录和实时数据进行不同的计算。
请注意，使用此变量/函数可能会导致指标重新绘制。
另见
barstate.isfirst
barstate.islast
barstate.ishistory
barstate.isrealtime
barstate.isconfirmed
barstate.islastconfirmedhistory
barstate.isrealtime

如果当前k线为实时k线，则返回true，否则返回false。
类型
series bool
备注
使用此变量的PineScript代码可以对历史记录和实时数据进行不同的计算。
请注意，使用此变量/函数可能会导致指标重新绘制。
另见
barstate.isfirst
barstate.islast
barstate.ishistory
barstate.isnew
barstate.isconfirmed
barstate.islastconfirmedhistory
box.all

返回一个数组，其中填充了脚本绘制的所有当前box。
类型
box[]
例子
//@version=5
indicator("box.all")
//delete all boxes
box.new(time, open, time + 60 * 60 * 24, close, xloc=xloc.bar_time, border_style=line.style_dashed)
a_allBoxes = box.all
if array.size(a_allBoxes) > 0
for i = 0 to array.size(a_allBoxes) - 1
box.delete(array.get(a_allBoxes, i))
备注
该阵列是只读的。阵列的索引零是图表上最旧对象的ID。
另见
box.new
line.all
label.all
table.all
chart.bg_color

从“图表设置/外观/背景”字段返回图表背景的颜色。选择渐变时，返回渐变的中点。
类型
input color
另见
chart.fg_color
chart.fg_color

返回与chart.bg_color提供最佳对比度的颜色。
类型
input color
另见
chart.bg_color
chart.is_heikinashi

类型
simple bool
返回值
如果图表类型是平均K线图，则返回{@on true}，否则返回{@on false}。
另见
chart.is_renko
chart.is_linebreak
chart.is_kagi
chart.is_pnf
chart.is_range
chart.is_kagi

类型
simple bool
返回值
如果图表类型是卡吉图，则返回true，否则返回false。
另见
chart.is_renko
chart.is_linebreak
chart.is_heikinashi
chart.is_pnf
chart.is_range
chart.is_linebreak

类型
simple bool
返回值
如果图表类型是新价线，则返回true，否则返回false。
另见
chart.is_renko
chart.is_heikinashi
chart.is_kagi
chart.is_pnf
chart.is_range
chart.is_pnf

类型
simple bool
返回值
如果图表类型是点数图，则返回true，否则返回false。
另见
chart.is_renko
chart.is_linebreak
chart.is_kagi
chart.is_heikinashi
chart.is_range
chart.is_range

类型
simple bool
返回值
如果图表类型是Range图，则返回true，否则返回false。
另见
chart.is_renko
chart.is_linebreak
chart.is_kagi
chart.is_pnf
chart.is_heikinashi
chart.is_renko

类型
simple bool
返回值
如果图表类型是砖形图，则返回{@on true}，否则返回{@on false}。
另见
chart.is_heikinashi
chart.is_linebreak
chart.is_kagi
chart.is_pnf
chart.is_range
chart.is_standard

类型
simple bool
返回值
如果图表类型是美国线、K线图、空心K线图、线形图、面积图或基准线线，则返回true，否则返回false。
另见
chart.is_renko
chart.is_linebreak
chart.is_kagi
chart.is_pnf
chart.is_range
chart.is_heikinashi
chart.left_visible_bar_time

图表上当前可见的最左侧K线的time。
类型
input int
备注
当其值更新以反映图表中的变化时，使用此变量的脚本将自动重新执行，这可能是由用户滚动图表或新的实时K线引起的。
另见
chart.right_visible_bar_time
chart.right_visible_bar_time

图表上当前可见的最右侧K线的time。
类型
input int
备注
当其值更新以反映图表中的变化时，使用此变量的脚本将自动重新执行，这可能是由用户滚动图表或新的实时K线引起的。
另见
chart.left_visible_bar_time
close

当前K线关闭时的收盘价，或尚未完成的实时K线的最后交易价格。
类型
series float
备注
可使用方括号运算符 []来访问以前的值，例如。 关[1]，关[2]。
另见
open
high
low
volume
time
hl2
hlc3
hlcc4
ohlc4
color.aqua

是#00BCD4颜色的命名常量。
类型
const color
另见
color.black
color.silver
color.gray
color.white
color.maroon
color.red
color.purple
color.fuchsia
color.green
color.lime
color.olive
color.yellow
color.navy
color.blue
color.teal
color.orange
color.black

是#363A45颜色的命名常量。
类型
const color
另见
color.silver
color.gray
color.white
color.maroon
color.red
color.purple
color.fuchsia
color.green
color.lime
color.olive
color.yellow
color.navy
color.blue
color.teal
color.aqua
color.orange
color.blue

是 #2962ff 颜色的命名常量。
类型
const color
另见
color.black
color.silver
color.gray
color.white
color.maroon
color.red
color.purple
color.fuchsia
color.green
color.lime
color.olive
color.yellow
color.navy
color.teal
color.aqua
color.orange
color.fuchsia

是#E040FB颜色的命名常量。
类型
const color
另见
color.black
color.silver
color.gray
color.white
color.maroon
color.red
color.purple
color.green
color.lime
color.olive
color.yellow
color.navy
color.blue
color.teal
color.aqua
color.orange
color.gray

是#787B86颜色的命名常量。
类型
const color
另见
color.black
color.silver
color.white
color.maroon
color.red
color.purple
color.fuchsia
color.green
color.lime
color.olive
color.yellow
color.navy
color.blue
color.teal
color.aqua
color.orange
color.green

是#4CAF50颜色的命名常量。
类型
const color
另见
color.black
color.silver
color.gray
color.white
color.maroon
color.red
color.purple
color.fuchsia
color.lime
color.olive
color.yellow
color.navy
color.blue
color.teal
color.aqua
color.orange
color.lime

是#00E676颜色的命名常量。
类型
const color
另见
color.black
color.silver
color.gray
color.white
color.maroon
color.red
color.purple
color.fuchsia
color.green
color.olive
color.yellow
color.navy
color.blue
color.teal
color.aqua
color.orange
color.maroon

为 ＃880E4F 颜色的命名常量。
类型
const color
另见
color.black
color.silver
color.gray
color.white
color.red
color.purple
color.fuchsia
color.green
color.lime
color.olive
color.yellow
color.navy
color.blue
color.teal
color.aqua
color.orange
color.navy

是#311B92颜色的命名常量。
类型
const color
另见
color.black
color.silver
color.gray
color.white
color.maroon
color.red
color.purple
color.fuchsia
color.green
color.lime
color.olive
color.yellow
color.blue
color.teal
color.aqua
color.orange
color.olive

是#808000颜色的命名常量。
类型
const color
另见
color.black
color.silver
color.gray
color.white
color.maroon
color.red
color.purple
color.fuchsia
color.green
color.lime
color.yellow
color.navy
color.blue
color.teal
color.aqua
color.orange
color.orange

是#FF9800颜色的命名常量。
类型
const color
另见
color.black
color.silver
color.gray
color.white
color.maroon
color.red
color.purple
color.fuchsia
color.green
color.lime
color.olive
color.yellow
color.navy
color.blue
color.teal
color.aqua
color.purple

是#9C27B0颜色的命名常量。
类型
const color
另见
color.black
color.silver
color.gray
color.white
color.maroon
color.red
color.fuchsia
color.green
color.lime
color.olive
color.yellow
color.navy
color.blue
color.teal
color.aqua
color.orange
color.red

是#FF5252颜色的命名常量。
类型
const color
另见
color.black
color.silver
color.gray
color.white
color.maroon
color.purple
color.fuchsia
color.green
color.lime
color.olive
color.yellow
color.navy
color.blue
color.teal
color.aqua
color.orange
color.silver

为 #B2B5BE 颜色的命名常量。
类型
const color
另见
color.black
color.gray
color.white
color.maroon
color.red
color.purple
color.fuchsia
color.green
color.lime
color.olive
color.yellow
color.navy
color.blue
color.teal
color.aqua
color.orange
color.teal

是#00897B颜色的命名常量。
类型
const color
另见
color.black
color.silver
color.gray
color.white
color.maroon
color.red
color.purple
color.fuchsia
color.green
color.lime
color.olive
color.yellow
color.navy
color.blue
color.aqua
color.orange
color.white

是#FFFFFF颜色的命名常量。
类型
const color
另见
color.black
color.silver
color.gray
color.maroon
color.red
color.purple
color.fuchsia
color.green
color.lime
color.olive
color.yellow
color.navy
color.blue
color.teal
color.aqua
color.orange
color.yellow

是#FFEB3B颜色的命名常量。
类型
const color
另见
color.black
color.silver
color.gray
color.white
color.maroon
color.red
color.purple
color.fuchsia
color.green
color.lime
color.olive
color.navy
color.blue
color.teal
color.aqua
color.orange
currency.AUD

澳元。
类型
const string
另见
strategy
currency.BTC

比特币。
类型
const string
另见
strategy
currency.CAD

加元
类型
const string
另见
strategy
currency.CHF

瑞士法郎
类型
const string
另见
strategy
currency.ETH

以太坊。
类型
const string
另见
strategy
currency.EUR

欧元.
类型
const string
另见
strategy
currency.GBP

英镑。
类型
const string
另见
strategy
currency.HKD

港币
类型
const string
另见
strategy
currency.JPY

日元
类型
const string
另见
strategy
currency.KRW

韩元。
类型
const string
另见
strategy
currency.MYR

马来西亚林吉特。
类型
const string
另见
strategy
currency.NOK

挪威克朗
类型
const string
另见
strategy
currency.NONE

未指明的货币。
类型
const string
另见
strategy
currency.NZD

新西兰元
类型
const string
另见
strategy
currency.RUB

俄罗斯卢布
类型
const string
另见
strategy
currency.SEK

瑞典克朗
类型
const string
另见
strategy
currency.SGD

新加坡元
类型
const string
另见
strategy
currency.TRY

土耳其里拉
类型
const string
另见
strategy
currency.USD

美元
类型
const string
另见
strategy
currency.USDT

Tether。
类型
const string
另见
strategy
currency.ZAR

南非兰特
类型
const string
另见
strategy
dayofmonth

交换时区的当前k线时间的日期。
类型
series int
备注
请注意，此变量根据K线的打开时间返回天。对于隔夜交易时段（例如EURUSD，其周一交易时段从周日17:00开始），该值可以比交易日的天低1。
另见
dayofmonth
time
year
month
weekofyear
dayofweek
hour
minute
second
dayofweek

交换时区的当前k线时间的星期。
类型
series int
备注
请注意，此变量根据K线的打开时间返回天。对于隔夜交易时段（例如EURUSD，其周一交易时段从周日17:00开始），该值可以比交易日的天低1。
您可以使用dayofweek.sunday，{@ var dayofweek.monday}，{@ var dayofweek.tuesday}，{@ var dayofweek.wednesday}，{@ var dayofweek.thursday}，{@ var dayofweek.friday} 和dayofweek.saturday变量用于比较。
另见
dayofweek
time
year
month
weekofyear
dayofmonth
hour
minute
second
dayofweek.friday

是{@dayofweek}函数的返回值和dayofweek变量的值的命名常量。
类型
const int
另见
dayofweek.sunday
dayofweek.monday
dayofweek.tuesday
dayofweek.wednesday
dayofweek.thursday
dayofweek.saturday
dayofweek.monday

是{@dayofweek}函数的返回值和dayofweek变量的值的命名常量。
类型
const int
另见
dayofweek.sunday
dayofweek.tuesday
dayofweek.wednesday
dayofweek.thursday
dayofweek.friday
dayofweek.saturday
dayofweek.saturday

是{@dayofweek}函数的返回值和dayofweek变量的值的命名常量。
类型
const int
另见
dayofweek.sunday
dayofweek.monday
dayofweek.tuesday
dayofweek.wednesday
dayofweek.thursday
dayofweek.friday
dayofweek.sunday

是{@dayofweek}函数的返回值和dayofweek变量的值的命名常量。
类型
const int
另见
dayofweek.monday
dayofweek.tuesday
dayofweek.wednesday
dayofweek.thursday
dayofweek.friday
dayofweek.saturday
dayofweek.thursday

是{@dayofweek}函数的返回值和dayofweek变量的值的命名常量。
类型
const int
另见
dayofweek.sunday
dayofweek.monday
dayofweek.tuesday
dayofweek.wednesday
dayofweek.friday
dayofweek.saturday
dayofweek.tuesday

是{@dayofweek}函数的返回值和dayofweek变量的值的命名常量。
类型
const int
另见
dayofweek.sunday
dayofweek.monday
dayofweek.wednesday
dayofweek.thursday
dayofweek.friday
dayofweek.saturday
dayofweek.wednesday

是{@dayofweek}函数的返回值和dayofweek变量的值的命名常量。
类型
const int
另见
dayofweek.sunday
dayofweek.monday
dayofweek.tuesday
dayofweek.thursday
dayofweek.friday
dayofweek.saturday
display.all

与 `display` 参数一起使用的命名参数。随处可见。
类型
plot_simple_display
另见
plot
plotshape
plotchar
plotarrow
plotbar
plotcandle
display.data_window

与 `display` 参数一起使用的命名参数。在数据窗口中显示绘图值，这是图表右侧栏中的一个可用菜单。
类型
plot_display
另见
plot
plotshape
plotchar
plotarrow
plotbar
plotcandle
display.none

与 `display` 参数一起使用的命名参数。导致不显示绘图值。尽管如此，绘制的值仍可用于警报模板消息，并将出现在导出的图表数据中。
类型
plot_simple_display
另见
plot
plotshape
plotchar
plotarrow
plotbar
plotcandle
display.pane

与 `display` 参数一起使用的命名参数。在脚本使用的窗格中显示绘图，由indicator/strategy声明语句的 `overlay` 参数定义。
类型
plot_display
另见
plot
plotshape
plotchar
plotarrow
plotbar
plotcandle
display.price_scale

与 `display` 参数一起使用的命名参数。如果图表设置允许，则控制价格坐标中图表标签和价格的显示。
类型
plot_display
另见
plot
plotshape
plotchar
plotarrow
plotbar
plotcandle
display.status_line

与 `display` 参数一起使用的命名参数。如果图表的设置允许，则在脚本的状态行中显示绘图值，在图表上的脚本名称旁边。
类型
plot_display
另见
plot
plotshape
plotchar
plotarrow
plotbar
plotcandle
dividends.gross

request.dividends函数的命名常量。用于请求扣除前股票的股息回报。
类型
const string
另见
request.dividends
dividends.net

request.dividends函数的命名常量。用于请求股票扣除后的股息回报。
类型
const string
另见
request.dividends
earnings.actual

request.earnings函数的命名常量。用于请求报告的收入值。
类型
const string
另见
request.earnings
earnings.estimate

request.earnings函数的命名常量。用于请求预估收益值。
类型
const string
另见
request.earnings
earnings.standardized

request.earnings函数的命名常量。用于请求标准化收益值。
类型
const string
另见
request.earnings
extend.both

line.new和line.set_extend函数的命名常量。
类型
const string
另见
line.new
line.set_extend
extend.none
extend.left
extend.right
extend.left

line.new和line.set_extend函数的命名常量。
类型
const string
另见
line.new
line.set_extend
extend.none
extend.right
extend.both
extend.none

line.new和line.set_extend函数的命名常量。
类型
const string
另见
line.new
line.set_extend
extend.left
extend.right
extend.both
extend.right

line.new和line.set_extend函数的命名常量。
类型
const string
另见
line.new
line.set_extend
extend.none
extend.left
extend.both
font.family_default

box.new, box.set_text_font_family, label.new, label.set_text_font_family, table.cell和table.cell_set_text_font_family功能的默认文本字体
类型
const string
另见
box.new
box.set_text_font_family
label.new
label.set_text_font_family
table.cell
table.cell_set_text_font_family
font.family_monospace
font.family_monospace

box.new, box.set_text_font_family, label.new, label.set_text_font_family, table.cell和table.cell_set_text_font_family功能的等宽文本字体
类型
const string
另见
box.new
box.set_text_font_family
label.new
label.set_text_font_family
table.cell
table.cell_set_text_font_family
font.family_default
format.inherit

是一个命名常量，用于在indicator函数中从父系列中选择脚本输出值的格式。
类型
const string
另见
indicator
format.price
format.volume
format.mintick

是与str.tostring函数一起使用的命名常量。使用此参数将数字传递给str.tostring将数字四舍五入到可以除以syminfo.mintick的最接近的值，没有余数，并向上取整，并返回带有尾随零的所述值的字符串版本。
类型
const string
另见
indicator
format.inherit
format.price
format.volume
format.percent

是一个命名常量，用于在指标函数中以百分比形式选择脚本输出值的格式。它在值后添加一个百分号。
类型
const string
备注
默认精度为2，与图表本身的精度无关。这可以通过indicator函数的'precision'参数进行更改。
另见
indicator
format.inherit
format.price
format.volume
format.price

是一个命名常量，用于在indicator函数中选择脚本输出值的格式作为价格。
类型
const string
备注
如果format是format.price，则设置默认精度值。您可以使用指标函数的precision参数来更改精度值。
另见
indicator
format.inherit
format.volume
format.volume

是一个命名常量，用于在indicator函数中选择脚本输出值的格式作为成交量，例如“5183”将被格式化为“5.183K”。
类型
const string
另见
indicator
format.inherit
format.price
high

当前最高价。
类型
series float
备注
可使用方括号运算符 []来访问以前的值，例如。 高[1]，高[2]。
另见
open
low
close
volume
time
hl2
hlc3
hlcc4
ohlc4
hl2

是(最高价 + 最低价)/2的快捷键
类型
series float
另见
open
high
low
close
volume
time
hlc3
hlcc4
ohlc4
hlc3

是(最高价+最低价+收盘价)/3的快捷键
类型
series float
另见
open
high
low
close
volume
time
hl2
hlcc4
ohlc4
hlcc4

是(高+低+收+收)/4的快捷键
类型
series float
另见
open
high
low
close
volume
time
hl2
hlc3
ohlc4
hline.style_dashed

是{@Hline}函数的点划线样式的命名常量。
类型
hline_style
另见
hline.style_solid
hline.style_dotted
hline.style_dotted

是{@Hline}函数的点虚线样式的命名常量。
类型
hline_style
另见
hline.style_solid
hline.style_dashed
hline.style_solid

是{@Hline}函数的实心线型的命名常量。
类型
hline_style
另见
hline.style_dotted
hline.style_dashed
hour

交易所时区的当前小时k线。
类型
series int
另见
hour
time
year
month
weekofyear
dayofmonth
dayofweek
minute
second
label.all

返回一个阵列，其中填充了脚本绘制的所有当前标签。
类型
label[]
例子
//@version=5
indicator("label.all")
//delete all labels
label.new(bar_index, close)
a_allLabels = label.all
if array.size(a_allLabels) > 0
for i = 0 to array.size(a_allLabels) - 1
label.delete(array.get(a_allLabels, i))
备注
该阵列是只读的。阵列的索引零是图表上最旧对象的ID。
另见
label.new
line.all
box.all
table.all
label.style_arrowdown

label.new和label.set_style函数的标签样式。
类型
const string
另见
label.new
label.set_style
label.set_textalign
label.style_none
label.style_xcross
label.style_cross
label.style_triangleup
label.style_triangledown
label.style_flag
label.style_circle
label.style_arrowup
label.style_label_up
label.style_label_down
label.style_label_left
label.style_label_right
label.style_label_lower_left
label.style_label_lower_right
label.style_label_upper_left
label.style_label_upper_right
label.style_label_center
label.style_square
label.style_diamond
label.style_arrowup

label.new和label.set_style函数的标签样式。
类型
const string
另见
label.new
label.set_style
label.set_textalign
label.style_none
label.style_xcross
label.style_cross
label.style_triangleup
label.style_triangledown
label.style_flag
label.style_circle
label.style_arrowdown
label.style_label_up
label.style_label_down
label.style_label_left
label.style_label_right
label.style_label_lower_left
label.style_label_lower_right
label.style_label_upper_left
label.style_label_upper_right
label.style_label_center
label.style_square
label.style_diamond
label.style_circle

label.new和label.set_style函数的标签样式。
类型
const string
另见
label.new
label.set_style
label.set_textalign
label.style_none
label.style_xcross
label.style_cross
label.style_triangleup
label.style_triangledown
label.style_flag
label.style_arrowup
label.style_arrowdown
label.style_label_up
label.style_label_down
label.style_label_left
label.style_label_right
label.style_label_lower_left
label.style_label_lower_right
label.style_label_upper_left
label.style_label_upper_right
label.style_label_center
label.style_square
label.style_diamond
label.style_cross

label.new和label.set_style函数的标签样式。
类型
const string
另见
label.new
label.set_style
label.set_textalign
label.style_none
label.style_xcross
label.style_triangleup
label.style_triangledown
label.style_flag
label.style_circle
label.style_arrowup
label.style_arrowdown
label.style_label_up
label.style_label_down
label.style_label_left
label.style_label_right
label.style_label_lower_left
label.style_label_lower_right
label.style_label_upper_left
label.style_label_upper_right
label.style_label_center
label.style_square
label.style_diamond
label.style_diamond

label.new和label.set_style函数的标签样式。
类型
const string
另见
label.new
label.set_style
label.set_textalign
label.style_none
label.style_xcross
label.style_cross
label.style_triangleup
label.style_triangledown
label.style_flag
label.style_circle
label.style_arrowup
label.style_arrowdown
label.style_label_up
label.style_label_down
label.style_label_left
label.style_label_right
label.style_label_lower_left
label.style_label_lower_right
label.style_label_upper_left
label.style_label_upper_right
label.style_label_center
label.style_square
label.style_flag

label.new和label.set_style函数的标签样式。
类型
const string
另见
label.new
label.set_style
label.set_textalign
label.style_none
label.style_xcross
label.style_cross
label.style_triangleup
label.style_triangledown
label.style_circle
label.style_arrowup
label.style_arrowdown
label.style_label_up
label.style_label_down
label.style_label_left
label.style_label_right
label.style_label_lower_left
label.style_label_lower_right
label.style_label_upper_left
label.style_label_upper_right
label.style_label_center
label.style_square
label.style_diamond
label.style_label_center

label.new和label.set_style函数的标签样式。
类型
const string
另见
label.new
label.set_style
label.set_textalign
label.style_none
label.style_xcross
label.style_cross
label.style_triangleup
label.style_triangledown
label.style_flag
label.style_circle
label.style_arrowup
label.style_arrowdown
label.style_label_up
label.style_label_down
label.style_label_left
label.style_label_right
label.style_label_lower_left
label.style_label_lower_right
label.style_label_upper_left
label.style_label_upper_right
label.style_square
label.style_diamond
label.style_label_down

label.new和label.set_style函数的标签样式。
类型
const string
另见
label.new
label.set_style
label.set_textalign
label.style_none
label.style_xcross
label.style_cross
label.style_triangleup
label.style_triangledown
label.style_flag
label.style_circle
label.style_arrowup
label.style_arrowdown
label.style_label_up
label.style_label_left
label.style_label_right
label.style_label_lower_left
label.style_label_lower_right
label.style_label_upper_left
label.style_label_upper_right
label.style_label_center
label.style_square
label.style_diamond
label.style_label_left

label.new和label.set_style函数的标签样式。
类型
const string
另见
label.new
label.set_style
label.set_textalign
label.style_none
label.style_xcross
label.style_cross
label.style_triangleup
label.style_triangledown
label.style_flag
label.style_circle
label.style_arrowup
label.style_arrowdown
label.style_label_up
label.style_label_down
label.style_label_right
label.style_label_lower_left
label.style_label_lower_right
label.style_label_upper_left
label.style_label_upper_right
label.style_label_center
label.style_square
label.style_diamond
label.style_label_lower_left

label.new和label.set_style函数的标签样式。
类型
const string
另见
label.new
label.set_style
label.set_textalign
label.style_none
label.style_xcross
label.style_cross
label.style_triangleup
label.style_triangledown
label.style_flag
label.style_circle
label.style_arrowup
label.style_arrowdown
label.style_label_up
label.style_label_down
label.style_label_left
label.style_label_right
label.style_label_lower_right
label.style_label_upper_left
label.style_label_upper_right
label.style_label_center
label.style_square
label.style_diamond
label.style_label_lower_right

label.new和label.set_style函数的标签样式。
类型
const string
另见
label.new
label.set_style
label.set_textalign
label.style_none
label.style_xcross
label.style_cross
label.style_triangleup
label.style_triangledown
label.style_flag
label.style_circle
label.style_arrowup
label.style_arrowdown
label.style_label_up
label.style_label_down
label.style_label_left
label.style_label_right
label.style_label_lower_left
label.style_label_upper_left
label.style_label_upper_right
label.style_label_center
label.style_square
label.style_diamond
label.style_label_right

label.new和label.set_style函数的标签样式。
类型
const string
另见
label.new
label.set_style
label.set_textalign
label.style_none
label.style_xcross
label.style_cross
label.style_triangleup
label.style_triangledown
label.style_flag
label.style_circle
label.style_arrowup
label.style_arrowdown
label.style_label_up
label.style_label_down
label.style_label_left
label.style_label_lower_left
label.style_label_lower_right
label.style_label_upper_left
label.style_label_upper_right
label.style_label_center
label.style_square
label.style_diamond
label.style_label_up

label.new和label.set_style函数的标签样式。
类型
const string
另见
label.new
label.set_style
label.set_textalign
label.style_none
label.style_xcross
label.style_cross
label.style_triangleup
label.style_triangledown
label.style_flag
label.style_circle
label.style_arrowup
label.style_arrowdown
label.style_label_down
label.style_label_left
label.style_label_right
label.style_label_lower_left
label.style_label_lower_right
label.style_label_upper_left
label.style_label_upper_right
label.style_label_center
label.style_square
label.style_diamond
label.style_label_upper_left

label.new和label.set_style函数的标签样式。
类型
const string
另见
label.new
label.set_style
label.set_textalign
label.style_none
label.style_xcross
label.style_cross
label.style_triangleup
label.style_triangledown
label.style_flag
label.style_circle
label.style_arrowup
label.style_arrowdown
label.style_label_up
label.style_label_down
label.style_label_left
label.style_label_right
label.style_label_lower_left
label.style_label_lower_right
label.style_label_upper_right
label.style_label_center
label.style_square
label.style_diamond
label.style_label_upper_right

label.new和label.set_style函数的标签样式。
类型
const string
另见
label.new
label.set_style
label.set_textalign
label.style_none
label.style_xcross
label.style_cross
label.style_triangleup
label.style_triangledown
label.style_flag
label.style_circle
label.style_arrowup
label.style_arrowdown
label.style_label_up
label.style_label_down
label.style_label_left
label.style_label_right
label.style_label_lower_left
label.style_label_lower_right
label.style_label_upper_left
label.style_label_center
label.style_square
label.style_diamond
label.style_none

label.new和label.set_style函数的标签样式。
类型
const string
另见
label.new
label.set_style
label.set_textalign
label.style_xcross
label.style_cross
label.style_triangleup
label.style_triangledown
label.style_flag
label.style_circle
label.style_arrowup
label.style_arrowdown
label.style_label_up
label.style_label_down
label.style_label_left
label.style_label_right
label.style_label_center
label.style_square
label.style_diamond
label.style_square

label.new和label.set_style函数的标签样式。
类型
const string
另见
label.new
label.set_style
label.set_textalign
label.style_none
label.style_xcross
label.style_cross
label.style_triangleup
label.style_triangledown
label.style_flag
label.style_circle
label.style_arrowup
label.style_arrowdown
label.style_label_up
label.style_label_down
label.style_label_left
label.style_label_right
label.style_label_lower_left
label.style_label_lower_right
label.style_label_upper_left
label.style_label_upper_right
label.style_label_center
label.style_diamond
label.style_text_outline

label.new和label.set_style函数的标签样式。
类型
const string
另见
label.new
label.set_style
label.set_textalign
label.style_none
label.style_xcross
label.style_cross
label.style_triangleup
label.style_triangledown
label.style_flag
label.style_circle
label.style_arrowup
label.style_arrowdown
label.style_label_up
label.style_label_down
label.style_label_left
label.style_label_right
label.style_label_lower_left
label.style_label_lower_right
label.style_label_upper_left
label.style_label_upper_right
label.style_label_center
label.style_square
label.style_diamond
label.style_triangledown

label.new和label.set_style函数的标签样式。
类型
const string
另见
label.new
label.set_style
label.set_textalign
label.style_none
label.style_xcross
label.style_cross
label.style_triangleup
label.style_flag
label.style_circle
label.style_arrowup
label.style_arrowdown
label.style_label_up
label.style_label_down
label.style_label_left
label.style_label_right
label.style_label_lower_left
label.style_label_lower_right
label.style_label_upper_left
label.style_label_upper_right
label.style_label_center
label.style_square
label.style_diamond
label.style_triangleup

label.new和label.set_style函数的标签样式。
类型
const string
另见
label.new
label.set_style
label.set_textalign
label.style_none
label.style_xcross
label.style_cross
label.style_triangledown
label.style_flag
label.style_circle
label.style_arrowup
label.style_arrowdown
label.style_label_up
label.style_label_down
label.style_label_left
label.style_label_right
label.style_label_lower_left
label.style_label_lower_right
label.style_label_upper_left
label.style_label_upper_right
label.style_label_center
label.style_square
label.style_diamond
label.style_xcross

label.new和label.set_style函数的标签样式。
类型
const string
另见
label.new
label.set_style
label.set_textalign
label.style_none
label.style_cross
label.style_triangleup
label.style_triangledown
label.style_flag
label.style_circle
label.style_arrowup
label.style_arrowdown
label.style_label_up
label.style_label_down
label.style_label_left
label.style_label_right
label.style_label_center
label.style_square
label.style_diamond
last_bar_index

最后一根图表K线的索引。K线索引以第一根K线为零开始。
类型
series int
例子
//@version=5
strategy("Mark Last X Bars For Backtesting", overlay = true, calc_on_every_tick = true)
lastBarsFilterInput = input.int(100, "Bars Count:")
// Here, we store the 'last_bar_index' value that is known from the beginning of the script's calculation.
// The 'last_bar_index' will change when new real-time bars appear, so we declare 'lastbar' with the 'var' keyword.
var lastbar = last_bar_index
// Check if the current bar_index is 'lastBarsFilterInput' removed from the last bar on the chart, or the chart is traded in real-time.
allowedToTrade = (lastbar - bar_index <= lastBarsFilterInput) or barstate.isrealtime
bgcolor(allowedToTrade ? color.new(color.green, 80) : na)
返回值
收盘的最后历史K线索引，或开盘的实时K线索引。
备注
请注意，使用此变量可能会导致指标重绘。
另见
bar_index
last_bar_time
barstate.ishistory
barstate.isrealtime
last_bar_time

最后一根图表K线的UNIX格式的时间。它是自1970年1月1日00:00:00 UTC以来经过的毫秒数。
类型
series int
备注
请注意，使用此变量/函数可能会导致指标重新绘制。
请注意，此变量根据K线的开盘时间返回时间戳。
另见
time
timenow
timestamp
last_bar_index
line.all

返回一个数组，其中填充了脚本绘制的所有当前线条。
类型
line[]
例子
//@version=5
indicator("line.all")
//delete all lines
line.new(bar_index - 10, close, bar_index, close)
a_allLines = line.all
if array.size(a_allLines) > 0
for i = 0 to array.size(a_allLines) - 1
line.delete(array.get(a_allLines, i))
备注
该阵列是只读的。阵列的索引零是图表上最旧对象的ID。
另见
line.new
label.all
box.all
table.all
line.style_arrow_both

line.new和line.set_style函数的线条样式。 两点上带箭头的实线。
类型
const string
另见
line.new
line.set_style
line.style_solid
line.style_dotted
line.style_dashed
line.style_arrow_left
line.style_arrow_right
line.style_arrow_left

line.new和line.set_style函数的线条样式。 第一点带箭头的实线。
类型
const string
另见
line.new
line.set_style
line.style_solid
line.style_dotted
line.style_dashed
line.style_arrow_right
line.style_arrow_both
line.style_arrow_right

line.new和line.set_style函数的线条样式。 第二点带箭头的实线。
类型
const string
另见
line.new
line.set_style
line.style_solid
line.style_dotted
line.style_dashed
line.style_arrow_left
line.style_arrow_both
line.style_dashed

line.new和line.set_style函数的线条样式。
类型
const string
另见
line.new
line.set_style
line.style_solid
line.style_dotted
line.style_arrow_left
line.style_arrow_right
line.style_arrow_both
line.style_dotted

line.new和line.set_style函数的线条样式。
类型
const string
另见
line.new
line.set_style
line.style_solid
line.style_dashed
line.style_arrow_left
line.style_arrow_right
line.style_arrow_both
line.style_solid

line.new和line.set_style函数的线条样式。
类型
const string
另见
line.new
line.set_style
line.style_dotted
line.style_dashed
line.style_arrow_left
line.style_arrow_right
line.style_arrow_both
linefill.all

返回一个由脚本绘制的所有当前linefill对象填充的阵列。
类型
linefill[]
备注
该阵列是只读的。阵列的索引零是图表上最旧对象的ID。
location.abovebar

plotshape，plotchar功能的位置值。 形状绘制在主系列k线上方。
类型
const string
另见
plotshape
plotchar
location.belowbar
location.top
location.bottom
location.absolute
location.absolute

plotshape，plotchar功能的位置值。 形状在图表上绘制，使用指标值作为价格坐标。
类型
const string
另见
plotshape
plotchar
location.abovebar
location.belowbar
location.top
location.bottom
location.belowbar

plotshape，plotchar功能的位置值。 形状绘制在主要系列k线下方。
类型
const string
另见
plotshape
plotchar
location.abovebar
location.top
location.bottom
location.absolute
location.bottom

plotshape，plotchar功能的位置值。 形状绘制在底部图表边框附近。
类型
const string
另见
plotshape
plotchar
location.abovebar
location.belowbar
location.top
location.absolute
location.top

plotshape，plotchar功能的位置值。 形状绘制在顶部图表边框附近。
类型
const string
另见
plotshape
plotchar
location.abovebar
location.belowbar
location.bottom
location.absolute
low

当前最低价。
类型
series float
备注
可使用方括号运算符 []来访问以前的值，例如。 低[1]，低[2]。
另见
open
high
close
volume
time
hl2
hlc3
hlcc4
ohlc4
math.e

是欧拉数)的命名常数。它等于2.7182818284590452。
类型
const float
另见
math.phi
math.pi
math.rphi
math.phi

是黄金分割的命名常数。等于1.6180339887498948。
类型
const float
另见
math.e
math.pi
math.rphi
math.pi

是阿基米德常数的命名常数。它等于3.1415926535897932。
类型
const float
另见
math.e
math.phi
math.rphi
math.rphi

是黄金分割率的命名常数。它等于0.6180339887498948。
