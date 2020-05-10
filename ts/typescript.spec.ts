import { createSourceFile, ScriptTarget } from 'typescript'

test("comment", () => {
  const sourceFile = createSourceFile(
    "foo.ts",
    "// comment",
    ScriptTarget.ES2020
  );

  console.log(sourceFile)
})

test.only("let statment", () => {
  const sourceFile = createSourceFile(
    "foo.ts",
    "let x = 1",
    ScriptTarget.ES2020
  );

  console.log(sourceFile.identifiers)
})
