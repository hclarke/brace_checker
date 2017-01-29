# brace_checker
tool to check that braces are balanced, and indented correctly

compilers will tell you that a brace is missing, but not where. this tool checks leading whitespace to help narrow down what the actual problem is.

consider the following fragment:

~~~~
void foo() {
  while(true)
    /* do stuff */
  }
}
~~~~

most compilers will give you an error at the final brace, since there's no matching open brace for it

this tool will give an error on the second last brace, because the matching open brace is at the wrong indentation level

# usage

pass the text to be checked via stdin

~~~~
cat somefile | brace_checker
~~~~
