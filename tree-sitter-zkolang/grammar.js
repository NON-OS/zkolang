// zKolang grammar for tree-sitter. Mirrors the compiler front end: a source file is a
// sequence of includes, constant and function definitions, and statements, over an
// expression language with the operators the parser accepts.
module.exports = grammar({
  name: 'zkolang',

  extras: $ => [/\s/, $.comment],

  rules: {
    source_file: $ => repeat($._item),

    _item: $ => choice(
      $.include,
      $.const_def,
      $.fn_def,
      $._statement,
    ),

    include: $ => seq('include', $.string, ';'),

    const_def: $ => seq('const', $.identifier, '=', choice($.number, $.array), ';'),

    fn_def: $ => seq('fn', $.identifier, '(', optional($._params), ')', '=', $._expr, ';'),
    _params: $ => seq($.identifier, repeat(seq(',', $.identifier))),

    _statement: $ => choice(
      $.let_stmt, $.input_stmt, $.secret_stmt, $.output_stmt, $.assert_stmt, $.for_stmt,
    ),
    let_stmt: $ => seq('let', $.identifier, '=', $._expr, ';'),
    input_stmt: $ => seq(choice('input', 'public'), $.identifier, ';'),
    secret_stmt: $ => seq(choice('secret', 'witness'), $.identifier, ';'),
    output_stmt: $ => seq(choice('output', 'reveal'), $._expr, ';'),
    assert_stmt: $ => seq(choice('assert', 'prove'), $._expr, ';'),
    for_stmt: $ => seq('for', $.identifier, 'in', $._expr, '..', $._expr,
      '{', repeat($._statement), '}'),

    _expr: $ => choice(
      $.binary, $.unary, $.call, $.index, $.array, $.inv, $.sel, $.if_expr,
      $.paren, $.number, $.identifier,
    ),

    binary: $ => choice(
      prec.left(1, seq($._expr, '||', $._expr)),
      prec.left(2, seq($._expr, '&&', $._expr)),
      prec.left(3, seq($._expr, choice('==', '!=', '<', '<=', '>', '>='), $._expr)),
      prec.left(4, seq($._expr, choice('+', '-'), $._expr)),
      prec.left(5, seq($._expr, choice('*', '/'), $._expr)),
    ),
    unary: $ => prec(6, seq(choice('-', '!'), $._expr)),
    call: $ => prec(7, seq($.identifier, '(', optional($._args), ')')),
    index: $ => prec(7, seq($.identifier, '[', $._expr, ']')),
    _args: $ => seq($._expr, repeat(seq(',', $._expr))),
    array: $ => seq('[', optional($._args), ']'),
    inv: $ => seq('inv', '(', $._expr, ')'),
    sel: $ => seq('sel', '(', $._expr, ',', $._expr, ',', $._expr, ')'),
    if_expr: $ => seq('if', $._expr, '{', $._expr, '}', 'else', '{', $._expr, '}'),
    paren: $ => seq('(', $._expr, ')'),

    identifier: $ => /[A-Za-z_][A-Za-z0-9_]*/,
    number: $ => /[0-9]+/,
    string: $ => /"[^"]*"/,
    comment: $ => token(seq('//', /.*/)),
  },
});
