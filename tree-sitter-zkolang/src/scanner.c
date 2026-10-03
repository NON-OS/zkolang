/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*
 * The one token a regular expression cannot read: a block comment, which nests
 * (SPEC section 2.1). Each `/*` opens a level and each `*\/` closes one; the comment ends
 * when the first level closes. An unterminated comment is not a token.
 */

#include "tree_sitter/parser.h"

enum TokenType { BLOCK_COMMENT };

void *tree_sitter_zkolang_external_scanner_create(void) { return NULL; }
void tree_sitter_zkolang_external_scanner_destroy(void *p) { (void)p; }
unsigned tree_sitter_zkolang_external_scanner_serialize(void *p, char *b) {
  (void)p;
  (void)b;
  return 0;
}
void tree_sitter_zkolang_external_scanner_deserialize(void *p, const char *b, unsigned n) {
  (void)p;
  (void)b;
  (void)n;
}

static void skip_space(TSLexer *lexer) {
  while (lexer->lookahead == ' ' || lexer->lookahead == '\t' || lexer->lookahead == '\n' ||
         lexer->lookahead == '\r') {
    lexer->advance(lexer, true);
  }
}

bool tree_sitter_zkolang_external_scanner_scan(void *p, TSLexer *lexer, const bool *valid) {
  (void)p;
  if (!valid[BLOCK_COMMENT]) return false;
  skip_space(lexer);
  if (lexer->lookahead != '/') return false;
  lexer->advance(lexer, false);
  if (lexer->lookahead != '*') return false;
  lexer->advance(lexer, false);
  unsigned depth = 1;
  while (depth > 0) {
    if (lexer->eof(lexer)) return false;
    int32_t c = lexer->lookahead;
    lexer->advance(lexer, false);
    if (c == '/' && lexer->lookahead == '*') {
      lexer->advance(lexer, false);
      depth++;
    } else if (c == '*' && lexer->lookahead == '/') {
      lexer->advance(lexer, false);
      depth--;
    }
  }
  lexer->result_symbol = BLOCK_COMMENT;
  return true;
}
