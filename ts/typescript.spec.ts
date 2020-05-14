import { createSourceFile, ScriptTarget } from 'typescript'

test("comment", () => {
  const sourceFile = createSourceFile(
    "foo.ts",
    "// comment",
    ScriptTarget.ES2020
  );

  console.log(sourceFile.statements)
  // SourceFileObject {
  //   pos: 0,
  //   end: 10,
  //   flags: 0,
  //   modifierFlagsCache: 0,
  //   transformFlags: 0,
  //   parent: undefined,
  //   kind: 290,
  //   text: '// comment',
  //   bindDiagnostics: [],
  //   bindSuggestionDiagnostics: undefined,
  //   languageVersion: 7,
  //   fileName: 'foo.ts',
  //   languageVariant: 0,
  //   isDeclarationFile: false,
  //   scriptKind: 3,
  //   pragmas: Map {},
  //   checkJsDirective: undefined,
  //   referencedFiles: [],
  //   typeReferenceDirectives: [],
  //   libReferenceDirectives: [],
  //   amdDependencies: [],
  //   hasNoDefaultLib: false,
  //   statements: [ pos: 0, end: 0 ],
  //   endOfFileToken: TokenObject {
  //     pos: 0,
  //     end: 10,
  //     flags: 0,
  //     modifierFlagsCache: 0,
  //     transformFlags: 0,
  //     parent: undefined,
  //     kind: 1
  //   },
  //   externalModuleIndicator: undefined,
  //   nodeCount: 2,
  //   identifierCount: 0,
  //   identifiers: Map {},
  //   parseDiagnostics: []
  // }
})

test("let statment", () => {
  const sourceFile = createSourceFile(
    "foo.ts",
    "let x = 1",
    ScriptTarget.ES2020
  );

  console.log(sourceFile)
  // SourceFileObject {
  //   pos: 0,
  //   end: 9,
  //   flags: 0,
  //   modifierFlagsCache: 0,
  //   transformFlags: 0,
  //   parent: undefined,
  //   kind: 290,
  //   text: 'let x = 1',
  //   bindDiagnostics: [],
  //   bindSuggestionDiagnostics: undefined,
  //   languageVersion: 7,
  //   fileName: 'foo.ts',
  //   languageVariant: 0,
  //   isDeclarationFile: false,
  //   scriptKind: 3,
  //   pragmas: Map {},
  //   checkJsDirective: undefined,
  //   referencedFiles: [],
  //   typeReferenceDirectives: [],
  //   libReferenceDirectives: [],
  //   amdDependencies: [],
  //   hasNoDefaultLib: false,
  //   statements: [
  //     NodeObject {
  //       pos: 0,
  //       end: 9,
  //       flags: 0,
  //       modifierFlagsCache: 0,
  //       transformFlags: 0,
  //       parent: undefined,
  //       kind: 225,
  //       declarationList: [NodeObject]
  //     },
  //     pos: 0,
  //     end: 9
  //   ],
  //   endOfFileToken: TokenObject {
  //     pos: 9,
  //     end: 9,
  //     flags: 0,
  //     modifierFlagsCache: 0,
  //     transformFlags: 0,
  //     parent: undefined,
  //     kind: 1
  //   },
  //   externalModuleIndicator: undefined,
  //   nodeCount: 7,
  //   identifierCount: 1,
  //   identifiers: Map { 'x' => 'x' },
  //   parseDiagnostics: []
  // }
})
