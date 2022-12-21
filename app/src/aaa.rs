Ok(Block {
stmts: [Exp(FuncCall(FunctionCall {
method: VarName(RVVarName { name: VarName
{ value: "study", range: StrRange
{ start: Position { line: 1, character: 0 },
end:
Position { line: 1, character: 5 } } },
var_index: VarIndex { varid: 0, rel_ctx: 0 },
eval_id: 0 }),
pos_args: [Str(StringNode { value: "Test", range: StrRange
{ start: Position { line: 1, character: 7 }, end:
Position { line: 1, character: 11 } } })],
dict_args: [], ctxid: 0, range: StrRange {
start: Position { line: 1, character: 0 },
end: Position { line: 1, character: 13 } },
func_type: None, spec_index: 0 })),
Assignment(Assignment { names: [VarName { value: "a", range: StrRange
{ start: Position { line: 3, character: 0 }, end: Position { line: 3, character: 1 } } }],
val: VarName(RVVarName { name: VarName { value: "close", range:
StrRange { start: Position { line: 3, character: 4 },
end: Position { line: 3, character: 9 } } },
var_index: VarIndex { varid: 0, rel_ctx: 0 }, eval_id: 0 }),
var_type: None, var: false,
range: StrRange { start: Position { line: 3, character: 0 },
end: Position { line: 3, character: 9 } },
varids: None, cast_index: VarIndex { varid: 0, rel_ctx: 0 }, cast_func_index: 0 }),
Exp(FuncCall(FunctionCall { method:
VarName(RVVarName { name: VarName { value: "plot",
range: StrRange { start: Position { line: 4, character: 0 },
end: Position { line: 4, character: 4 } } },
var_index: VarIndex { varid: 0, rel_ctx: 0 }, eval_id: 0 }),
pos_args: [VarName(RVVarName { name: VarName { value: "a", range:
StrRange { start: Position { line: 4, character: 5 },
end: Position { line: 4, character: 6 } } }, var_index:
VarIndex { varid: 0, rel_ctx: 0 }, eval_id: 0 })],
dict_args: [], ctxid: 0, range: StrRange { start: Position { line: 4, character: 0 },
end: Position { line: 4, character: 7 } }, func_type: None, spec_index: 0 }))],
ret_stmt: None, range: StrRange { start: Position { line: 1, character: 0 },
end: Position { line: 4, character: 7 } }, var_count: 0, libfun_count: 0, subctx_count: 0 })
