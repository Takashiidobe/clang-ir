//! Structured and block-level control-flow operations.

/// `cir.br`
/// Unconditional branch
///
/// The `cir.br` branches unconditionally to a block. Used to represent C/C++
/// goto's and general block branching.
///
/// Note that for source level `goto`'s crossing scope boundaries, those are
/// usually represented with the "symbolic" `cir.goto` operation.
///
/// Example:
///
/// ```
///   ...
///     cir.br ^bb3
///   ^bb3:
///     cir.return
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Br {
    /// variadic of CIR void type or CIR bool type or CIR array type or CIR vector type or Integer type with arbitrary precision up to a fixed limit or single float type or double float type or f16 type or bf16 type or f80 type or f128 type or long double type or CIR pointer type or CIR function type or CIR struct/class type or CIR union type or CIR complex type or CIR type that is used for the vptr member of C++ objects or CIR type that represents a pointer-to-data-member in C++ or CIR type that represents C++ pointer-to-member-function type or CIR exception handling token type or CIR cleanup token type or CIR catch token type
    pub dest_operands: Vec<super::ValueId>,
    pub successors: Vec<String>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.brcond`
/// Conditional branch
///
/// The `cir.brcond %cond, ^bb0, ^bb1` branches to 'bb0' block in case
/// %cond (which must be a !cir.bool type) evaluates to true, otherwise
/// it branches to 'bb1'.
///
/// Example:
///
/// ```
///   ...
///     cir.brcond %a, ^bb3, ^bb4
///   ^bb3:
///     cir.return
///   ^bb4:
///     cir.yield
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Brcond {
    /// CIR bool type
    pub cond: super::ValueId,
    /// variadic of CIR void type or CIR bool type or CIR array type or CIR vector type or Integer type with arbitrary precision up to a fixed limit or single float type or double float type or f16 type or bf16 type or f80 type or f128 type or long double type or CIR pointer type or CIR function type or CIR struct/class type or CIR union type or CIR complex type or CIR type that is used for the vptr member of C++ objects or CIR type that represents a pointer-to-data-member in C++ or CIR type that represents C++ pointer-to-member-function type or CIR exception handling token type or CIR cleanup token type or CIR catch token type
    pub dest_operands_true: Vec<super::ValueId>,
    /// variadic of CIR void type or CIR bool type or CIR array type or CIR vector type or Integer type with arbitrary precision up to a fixed limit or single float type or double float type or f16 type or bf16 type or f80 type or f128 type or long double type or CIR pointer type or CIR function type or CIR struct/class type or CIR union type or CIR complex type or CIR type that is used for the vptr member of C++ objects or CIR type that represents a pointer-to-data-member in C++ or CIR type that represents C++ pointer-to-member-function type or CIR exception handling token type or CIR cleanup token type or CIR catch token type
    pub dest_operands_false: Vec<super::ValueId>,
    pub successors: Vec<String>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.break`
/// C/C++ `break` statement equivalent
///
/// The `cir.break` operation is used to cease the execution of the current loop
/// or switch operation and transfer control to the parent operation. It is only
/// allowed within a breakable operations (loops and switches).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Break {
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.case`
/// Case operation
///
/// The `cir.case` operation represents a case within a C/C++ switch.
/// The `cir.case` operation must be in a `cir.switch` operation directly
/// or indirectly.
///
/// The `cir.case` have 4 kinds:
/// - `equal, <constant>`: equality of the second case operand against the
/// condition.
/// - `anyof, [constant-list]`: equals to any of the values in a subsequent
/// following list.
/// - `range, [lower-bound, upper-bound]`: the condition is within the closed
///                                        interval.
/// - `default`: any other value.
///
/// Each case region must be explicitly terminated.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Case {
    /// array attribute
    pub value: crate::attrs::Attribute,
    /// case kind
    pub kind: crate::enums::CaseOpKind,
    pub case_region: super::Region,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.cleanup.scope`
/// Represents a scope with associated cleanup code
///
/// `cir.cleanup.scope` contains a body region and a cleanup region. The body
/// region is executed first, and the cleanup region is executed when the body
/// region is exited, either normally or due to an exception.
///
/// The cleanup kind attribute specifies when the cleanup region should be
/// executed:
/// - `none`: No cleanup (cleanup region is empty/unused)
/// - `normal`: Cleanup is executed only on normal exit
/// - `eh`: Cleanup is executed only on exception unwinding
/// - `all`: Cleanup is executed on both normal exit and exception unwinding
///
/// Examples:
///
/// ```
/// // Cleanup that runs on both normal and exception paths
/// cir.cleanup.scope {
///   cir.call @mayThrow() : () -> ()
///   cir.yield
/// } cleanup all {
///   cir.call @destructor() : () -> ()
///   cir.yield
/// }
///
/// // EH-only cleanup (destructor only called on exception)
/// cir.cleanup.scope {
///   cir.call @mayThrow() : () -> ()
///   cir.yield
/// } cleanup eh {
///   cir.call @destructor() : () -> ()
///   cir.yield
/// }
/// ```
///
/// Both regions must be terminated. If a region has only one block, the
/// terminator can be left out, and `cir.yield` will be inserted implicitly.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CleanupScope {
    /// Cleanup kind attribute
    pub cleanup_kind: crate::enums::CleanupKind,
    pub body_region: super::Region,
    pub cleanup_region: super::Region,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.condition`
/// Loop continuation condition.
///
/// The `cir.condition` terminates conditional regions. It takes a single
/// `cir.bool` operand and, depending on its value, may branch to different
/// regions:
///
///  - When in the `cond` region of a loop, it continues the loop
///    if true, or exits it if false.
///  - When in the `ready` region of a `cir.await`, it branches to the `resume`
///    region when true, and to the `suspend` region when false.
///
/// Example:
///
/// ```
/// cir.for cond {
///   cir.condition(%val) // Branches to `step` region or exits.
/// } body {
///   cir.yield
/// } step {
///   cir.yield
/// }
///
/// cir.await(user, ready : {
///   cir.condition(%arg0) // Branches to `resume` or `suspend` region.
/// }, suspend : {
///   [...]
/// }, resume : {
///   [...]
/// },)
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Condition {
    /// CIR bool type
    pub condition: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.continue`
/// C/C++ `continue` statement equivalent
///
/// The `cir.continue` operation is used to end execution of the current
/// iteration of a loop and resume execution beginning at the next iteration.
/// It is only allowed within loop regions.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Continue {
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.do`
/// C/C++ do-while loop
///
/// Represents a C/C++ do-while loop. Identical to `cir.while` but the
/// condition is evaluated after the body. Because a variable cannot be
/// declared in the condition of a do-while loop, this operation cannot
/// have a `cleanup` region. A cleanup scope should be created within the
/// body region for any variables within the loop that require cleanup.
///
/// Example:
///
/// ```
/// cir.do {
///   cir.break
/// ^bb2:
///   cir.yield
/// } while {
///   cir.condition %cond : cir.bool
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Do {
    pub body: super::Region,
    pub cond: super::Region,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.for`
/// C/C++ for loop counterpart
///
/// Represents a C/C++ for loop. It consists of three or four regions:
///
///  - `cond`: single block region with the loop's condition. Should be
///  terminated with a `cir.condition` operation.
///  - `body`: contains the loop body and an arbitrary number of blocks.
///  - `step`: single block region with the loop's step.
///  - `cleanup`: optional region that runs on every per-iteration exit edge
///  (condition-false exit, end-of-iteration after the step, break/continue,
///  and EH unwinding when the cleanup kind includes EH). This is used to
///  destroy a condition variable whose lifetime is a single iteration. When
///  present, it carries a cleanup kind matching `cir.cleanup.scope` (`normal`
///  or `all`).
///
/// Example:
///
/// ```
/// cir.for cond {
///   cir.condition(%val)
/// } body {
///   cir.break
/// ^bb2:
///   cir.yield
/// } step {
///   cir.yield
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct For {
    /// Cleanup kind attribute
    pub cleanup_kind: Option<crate::enums::CleanupKind>,
    pub cond: super::Region,
    pub body: super::Region,
    pub step: super::Region,
    pub cleanup: super::Region,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.goto`
///
///
/// Transfers control to the specified `label`. This requires a corresponding
/// `cir.label` to exist and is used by to represent source level `goto`s
/// that jump across region boundaries. Alternatively, `cir.br` is used to
/// construct goto's that don't violate such boundaries.
///
/// `cir.goto` is completely symbolic (i.e. it "jumps" on a label that isn't
/// yet materialized) and should be taken into account by passes and analysis
/// when deciding if it's safe to make some assumptions about a given region
/// or basic block.
///
/// Example:
/// ```C++
///   int test(int x) {
///     if (x)
///       goto label;
///     {
///       x = 10;
///   label:
///       return x;
///     }
///   }
/// ```
///
/// ```
///   cir.scope {  // REGION #1
///     %2 = cir.load %0 : !cir.ptr<!s32i>, !s32i
///     %3 = cir.cast int_to_bool %2 : !s32i -> !cir.bool
///     cir.if %3 {
///       cir.goto "label"
///     }
///     }
///     cir.scope {  // REGION #2
///       %2 = cir.const #cir.int<10> : !s32i
///       cir.store %2, %0 : !s32i, !cir.ptr<!s32i>
///       cir.br ^bb1
///     ^bb1:  // pred: ^bb0
///       cir.label "label"
///       %3 = cir.load %0 : !cir.ptr<!s32i>, !s32i
///       cir.store %3, %1 : !s32i, !cir.ptr<!s32i>
///       %4 = cir.load %1 : !cir.ptr<!s32i>, !s32i
///       cir.return %4 : !s32i
///     }
///     cir.unreachable
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Goto {
    /// string attribute
    pub label: String,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.if`
/// the if-then-else operation
///
/// The `cir.if` operation represents an if-then-else construct for
/// conditionally executing two regions of code. The operand is a `cir.bool`
/// type.
///
/// Examples:
///
/// ```
/// cir.if %cond  {
///   ...
/// } else {
///   ...
/// }
///
/// cir.if %cond  {
///   ...
/// }
///
/// cir.if %cond  {
///   ...
///   cir.br ^a
/// ^a:
///   cir.yield
/// }
/// ```
///
/// `cir.if` defines no values and the 'else' can be omitted. The if/else
/// regions must be terminated. If the region has only one block, the terminator
/// can be left out, and `cir.yield` terminator will be inserted implictly.
/// Otherwise, the region must be explicitly terminated.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct If {
    /// CIR bool type
    pub condition: super::ValueId,
    pub then_region: super::Region,
    pub else_region: super::Region,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.indirect_goto`
/// Symbolic indirect goto
///
/// Transfers control to the block whose address is held in `$addr`, the
/// void-pointer value of a `goto *expr;` (the GNU computed-goto extension).
/// Like `cir.goto`, it is symbolic: it references no successor block, so it is
/// valid inside any region even before `FlattenCFG` merges nested scopes.
/// `GotoSolver` runs after `FlattenCFG` and rewrites each `cir.indirect_goto`
/// into a `cir.br` to a shared block holding a `cir.indirect_br` over every
/// address-taken label.
///
/// Example:
///
/// ```mlir
///   %0 = cir.load %p : !cir.ptr<!cir.ptr<!void>>, !cir.ptr<!void>
///   cir.indirect_goto %0 : !cir.ptr<!void>
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IndirectGoto {
    /// pointer to void type
    pub addr: super::ValueId,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.label`
///
/// An identifier which may be referred by cir.goto operation
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Label {
    /// string attribute
    pub label: String,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.return`
/// Return from function
///
/// The "return" operation represents a return operation within a function.
/// The operation takes an optional operand and produces no results.
/// The operand type must match the signature of the function that contains
/// the operation.
///
/// ```
///   func @foo() -> i32 {
///     ...
///     cir.return %0 : i32
///   }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Return {
    /// variadic of CIR void type or CIR bool type or CIR array type or CIR vector type or Integer type with arbitrary precision up to a fixed limit or single float type or double float type or f16 type or bf16 type or f80 type or f128 type or long double type or CIR pointer type or CIR function type or CIR struct/class type or CIR union type or CIR complex type or CIR type that is used for the vptr member of C++ objects or CIR type that represents a pointer-to-data-member in C++ or CIR type that represents C++ pointer-to-member-function type or CIR exception handling token type or CIR cleanup token type or CIR catch token type
    pub input: Vec<super::ValueId>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.scope`
/// Represents a C/C++ scope
///
/// `cir.scope` contains one region and defines a strict "scope" for all new
/// values produced within its blocks.
///
/// The region can contain an arbitrary number of blocks but usually defaults
/// to one and can optionally return a value (useful for representing values
/// coming out of C++ full-expressions) via `cir.yield`:
///
///
/// ```
/// %rvalue = cir.scope {
///   ...
///   cir.yield %value
/// }
/// ```
///
/// The blocks can be terminated by `cir.yield`, `cir.return` or `cir.throw`.
/// If `cir.scope` yields no value, the `cir.yield` can be left out, and
/// will be inserted implicitly.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Scope {
    pub results: Option<super::ValueId>,
    pub results_ty: Option<crate::types::Type>,
    pub scope_region: super::Region,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.switch`
/// Switch operation
///
/// The `cir.switch` operation represents C/C++ switch functionality for
/// conditionally executing multiple regions of code. The operand to an switch
/// is an integral condition value.
///
/// Besides taking an integer condition and CIR regions, it also accepts an
/// `all_enum_cases_covered` attribute indicating whether all enum cases are
/// handled by the operation. Note that the presence of a default CaseOp does
/// not imply `all_enum_cases_covered`. The original AST switch must explicitly list
/// every enum case.
///
/// The set of `cir.case` operations and their enclosing `cir.switch`
/// represent the semantics of a C/C++ switch statement. Users can use
/// `collectCases(llvm::SmallVector<CaseOp> &cases)` to collect the `cir.case`
/// operation in the `cir.switch` operation easily.
///
/// The `cir.case` operations don't have to be in the region of `cir.switch`
/// directly. However, when all the `cir.case` operations live in the region
/// of `cir.switch` directly and there are no other operations except the ending
/// `cir.yield` operation in the region of `cir.switch` directly, we say the
/// `cir.switch` operation is in a simple form. Users can use
/// `bool isSimpleForm(llvm::SmallVector<CaseOp> &cases)` member function to
/// detect if the `cir.switch` operation is in a simple form. The simple form
/// makes it easier for analyses to handle the `cir.switch` operation
/// and makes the boundary to give up clear.
///
/// To make the simple form as common as possible, CIR code generation attaches
/// operations corresponding to the statements that lives between top level
/// cases into the closest `cir.case` operation.
///
/// For example,
///
/// ```
/// switch(int cond) {
///   case 4:
///     a++;
///     b++;
///   case 5:
///     c++;
///
///   ...
/// }
/// ```
///
/// The statement `b++` is not a sub-statement of the case statement `case 4`.
/// But to make the generated `cir.switch` a simple form, we will attach the
/// statement `b++` into the closest `cir.case` operation. So that the generated
/// code will be like:
///
/// ```
/// cir.switch(int cond) {
///   cir.case(equal, 4) {
///     a++;
///     b++;
///     cir.yield
///   }
///   cir.case(equal, 5) {
///     c++;
///     cir.yield
///   }
///   ...
/// }
/// ```
///
/// For the same reason, we will hoist the case statement as the substatement
/// of another case statement so that they will be in the same level. For
/// example,
///
/// ```
/// switch(int cond) {
///   case 4:
///   default;
///   case 5:
///     a++;
///   ...
/// }
/// ```
///
/// will be generated as
///
/// ```
/// cir.switch(int cond) {
///   cir.case(equal, 4) {
///     cir.yield
///   }
///   cir.case(default) {
///     cir.yield
///   }
///   cir.case(equal, 5) {
///     a++;
///     cir.yield
///   }
///   ...
/// }
/// ```
///
/// The cir.switch is not be considered "simple" if any of the following is
/// true:
/// - There are case statements of the switch statement that are scope
///   other than the top level compound statement scope. Note that a case
///   statement itself doesn't form a scope.
/// - The sub-statement of the switch statement is not a compound statement.
/// - There is any code before the first case statement. For example,
///
/// ```
/// switch(int cond) {
///   l:
///     b++;
///
///   case 4:
///     a++;
///     break;
///
///   case 5:
///     goto l;
///   ...
/// }
/// ```
///
/// the generated CIR for this non-simple switch would be:
///
/// ```
/// cir.switch(int cond) {
///   cir.label "l"
///   b++;
///   cir.case(4) {
///     a++;
///     cir.break
///   }
///   cir.case(5) {
///     goto "l"
///   }
///   cir.yield
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Switch {
    /// Integer type with arbitrary precision up to a fixed limit
    pub condition: super::ValueId,
    /// unit attribute
    pub all_enum_cases_covered: bool,
    pub body: super::Region,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.switch.flat`
/// A flattened version of cir.switch
///
/// The `cir.switch.flat` operation is a region-less and simplified
/// version of the `cir.switch`.
/// Its representation is closer to LLVM IR dialect
/// than the C/C++ language feature.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SwitchFlat {
    /// Integer type with arbitrary precision up to a fixed limit
    pub condition: super::ValueId,
    /// variadic of any non-token type
    pub default_operands: Vec<super::ValueId>,
    /// variadic of any non-token type
    pub case_operands: Vec<Vec<super::ValueId>>,
    /// array attribute
    pub case_values: crate::attrs::Attribute,
    /// i32 dense array attribute
    pub case_operand_segments: crate::attrs::Attribute,
    pub successors: Vec<String>,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.ternary`
/// The `cond ? a : b` C/C++ ternary operation
///
/// The `cir.ternary` operation represents C/C++ ternary, much like a `select`
/// operation. The first argument is a `cir.bool` condition to evaluate, followed
/// by two regions to execute (true or false). This is different from `cir.if`
/// since each region is one block sized and the `cir.yield` closing the block
/// scope should have one argument.
///
/// `cir.ternary` also represents the GNU binary conditional operator ?: which
/// reuses the parent operation for both the condition and the true branch to
/// evaluate it only once.
///
/// Example:
///
/// ```
/// // cond = a && b;
///
/// %x = cir.ternary (%cond, true_region {
///   ...
///   cir.yield %a : i32
/// }, false_region {
///   ...
///   cir.yield %b : i32
/// }) -> i32
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Ternary {
    pub result: Option<super::ValueId>,
    pub result_ty: Option<crate::types::Type>,
    /// CIR bool type
    pub cond: super::ValueId,
    pub true_region: super::Region,
    pub false_region: super::Region,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.trap`
/// Exit the program abnormally
///
/// The cir.trap operation causes the program to exit abnormally. The
/// implementations may implement this operation with different mechanisms. For
/// example, an implementation may implement this operation by calling abort,
/// while another implementation may implement this operation by executing an
/// illegal instruction.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Trap {
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.unreachable`
/// invoke immediate undefined behavior
///
/// If the program control flow reaches a `cir.unreachable` operation, the
/// program exhibits undefined behavior immediately. This operation is useful
/// in cases where the unreachability of a program point needs to be explicitly
/// marked.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Unreachable {
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.while`
/// C/C++ while loop
///
/// Represents a C/C++ while loop. It consists of two or three regions:
///
///  - `cond`: single block region with the loop's condition. Should be
///  terminated with a `cir.condition` operation.
///  - `body`: contains the loop body and an arbitrary number of blocks.
///  - `cleanup`: optional region that runs on every per-iteration exit edge
///  (condition-false exit, end-of-iteration, break/continue, and EH unwinding
///  when the cleanup kind includes EH). This is used to destroy a condition
///  variable whose lifetime is a single iteration. When present, it carries a
///  cleanup kind matching `cir.cleanup.scope` (`normal` or `all`). Note that
///  a `DoWhileOp` cannot contain a `cleanup` region.
///
/// Example:
///
/// ```
/// cir.while {
///   cir.break
/// ^bb2:
///   cir.yield
/// } do {
///   cir.condition %cond : cir.bool
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct While {
    /// Cleanup kind attribute
    pub cleanup_kind: Option<crate::enums::CleanupKind>,
    pub cond: super::Region,
    pub body: super::Region,
    pub cleanup: super::Region,
    pub loc: Option<crate::ast::SourceLocation>,
}
/// `cir.yield`
/// Represents the default branching behaviour of a region
///
/// The `cir.yield` operation terminates regions on different CIR operations,
/// and it is used to represent the default branching behaviour of a region.
/// Said branching behaviour is determinted by the parent operation. For
/// example, a yield in a `switch-case` region implies a fallthrough, while
/// a yield in a `cir.if` region implies a branch to the exit block, and so
/// on.
///
/// In some cases, it might yield an SSA value and the semantics of how the
/// values are yielded is defined by the parent operation. For example, a
/// `cir.ternary` operation yields a value from one of its regions.
///
/// As a general rule, `cir.yield` must be explicitly used whenever a region has
/// more than one block and no terminator, or within `cir.switch` regions not
/// `cir.return` terminated.
///
/// Examples:
/// ```
/// cir.if %4 {
///   ...
///   cir.yield
/// }
///
/// cir.switch (%5) [
///   case (equal, 3) {
///     ...
///     cir.yield
///   }, ...
/// ]
///
/// cir.scope {
///   ...
///   cir.yield
/// }
///
/// %x = cir.scope {
///   ...
///   cir.yield %val
/// }
///
/// %y = cir.ternary {
///   ...
///   cir.yield %val : i32
/// } : i32
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Yield {
    /// variadic of CIR void type or CIR bool type or CIR array type or CIR vector type or Integer type with arbitrary precision up to a fixed limit or single float type or double float type or f16 type or bf16 type or f80 type or f128 type or long double type or CIR pointer type or CIR function type or CIR struct/class type or CIR union type or CIR complex type or CIR type that is used for the vptr member of C++ objects or CIR type that represents a pointer-to-data-member in C++ or CIR type that represents C++ pointer-to-member-function type or CIR exception handling token type or CIR cleanup token type or CIR catch token type
    pub args: Vec<super::ValueId>,
    pub loc: Option<crate::ast::SourceLocation>,
}