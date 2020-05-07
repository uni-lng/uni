import { createSourceFile, ScriptTarget } from 'typescript'

test("comment", () => {
  const sourceFile = createSourceFile(
    "foo.ts",
    "// comment",
    ScriptTarget.ES2020
  );

  console.log(sourceFile)
})
